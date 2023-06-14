//! Copyright (c) 2023 MASSA LABS <info@massa.net>

use std::collections::{btree_map::Entry, BTreeMap};
use tracing::{debug, info};

use massa_models::denunciation::DenunciationIndex;
use massa_models::slot::Slot;
use massa_models::{
    address::Address,
    denunciation::{Denunciation, DenunciationPrecursor},
    timeslots::get_closest_slot_to_timestamp,
};
use massa_pool_exports::{PoolChannels, PoolConfig};
use massa_storage::Storage;
use massa_time::MassaTime;

pub struct DenunciationPool {
    /// pool configuration
    config: PoolConfig,
    /// pool channels
    channels: PoolChannels,
    /// last consensus final periods, per thread
    last_cs_final_periods: Vec<u64>,
    /// Internal cache for denunciations
    denunciations_cache: BTreeMap<DenunciationIndex, DenunciationStatus>,
}

impl DenunciationPool {
    pub fn init(config: PoolConfig, channels: PoolChannels) -> Self {
        Self {
            config,
            channels,
            last_cs_final_periods: vec![0u64; config.thread_count as usize],
            denunciations_cache: Default::default(),
        }
    }

    /// Get the number of stored elements
    pub fn len(&self) -> usize {
        self.denunciations_cache
            .iter()
            .filter(|(_, de_st)| matches!(*de_st, DenunciationStatus::DenunciationEmitted(..)))
            .count()
    }

    /// Checks whether an element is stored in the pool - only used in unit tests for now
    #[cfg(feature = "testing")]
    pub fn contains(&self, denunciation: &Denunciation) -> bool {
        self.denunciations_cache
            .iter()
            .find(|(_, de_st)| match *de_st {
                DenunciationStatus::Accumulating(_) => false,
                DenunciationStatus::DenunciationEmitted(de) => de == denunciation,
            })
            .is_some()
    }

    /// Add a denunciation precursor to the pool - can lead to a Denunciation creation
    /// Note that the Denunciation is stored in the denunciation pool internal cache
    pub fn add_denunciation_precursor(&mut self, denunciation_precursor: DenunciationPrecursor) {
        let slot = denunciation_precursor.get_slot();

        // Do some checkups before adding the denunciation precursor

        if slot.period <= self.config.last_start_period {
            // denunciation created before last restart (can be 0 or >= 0 after a network restart) - ignored
            // Note: as we use '<=', also ignore denunciation created for genesis block
            return;
        }

        let now = MassaTime::now().expect("could not get current time");

        // get closest slot according to the current absolute time
        let slot_now = get_closest_slot_to_timestamp(
            self.config.thread_count,
            self.config.t0,
            self.config.genesis_timestamp,
            now,
        );

        // Note about last_cs_final_periods.iter().min()
        // Unlike operations, denunciations can be included in any thread
        // So Denunciations can only be expired when they cannot be included in any thread
        if Denunciation::is_expired(
            &slot.period,
            self.last_cs_final_periods.iter().min().unwrap_or(&0),
            &self.config.denunciation_expire_periods,
        ) {
            // too old - cannot be denounced anymore
            return;
        }

        if slot.period.saturating_sub(slot_now.period) > self.config.denunciation_expire_periods {
            // too much in the future - ignored
            return;
        }

        // Get selected address from selector and check
        // Note: If the public key of the header creator is not checked to match the PoS,
        //       someone can spam with headers coming from various non-PoS-drawn pubkeys
        //       and cause a problem
        match &denunciation_precursor {
            DenunciationPrecursor::Endorsement(de_p) => {
                // Get selected address from selector and check
                let selected = self.channels.selector.get_selection(de_p.slot);
                match selected {
                    Ok(selection) => {
                        if let Some(address) = selection.endorsements.get(de_p.index as usize) {
                            let a = Address::from_public_key(&de_p.public_key);
                            if *address != a {
                                debug!("Denunciation pool received a secure share endorsement but address was not selected: received {} but expected {} ({})", address, a, de_p.public_key);
                                return;
                            }
                        } else {
                            debug!("Denunciation pool could not get selected address for endorsements at index");
                            return;
                        }
                    }
                    Err(e) => {
                        debug!("Cannot get producer from selector: {}", e);
                        return;
                    }
                }
            }
            DenunciationPrecursor::BlockHeader(de_p) => {
                let selected_address = self.channels.selector.get_producer(de_p.slot);
                match selected_address {
                    Ok(address) => {
                        if address
                            != Address::from_public_key(denunciation_precursor.get_public_key())
                        {
                            debug!("Denunciation pool received a secured header but address was not selected");
                            return;
                        }
                    }
                    Err(e) => {
                        debug!("Cannot get producer from selector: {}", e);
                        return;
                    }
                }
            }
        }

        let key = DenunciationIndex::from(&denunciation_precursor);

        let denunciation_: Option<Denunciation> = match self.denunciations_cache.entry(key) {
            Entry::Occupied(mut eo) => match eo.get_mut() {
                DenunciationStatus::Accumulating(de_p_) => {
                    let de_p: &DenunciationPrecursor = de_p_;
                    if *de_p != denunciation_precursor {
                        match Denunciation::try_from((de_p, &denunciation_precursor)) {
                            Ok(de) => {
                                eo.insert(DenunciationStatus::DenunciationEmitted(de.clone()));
                                Some(de)
                            }
                            Err(e) => {
                                debug!("Denunciation pool cannot create denunciation from endorsements: {}", e);
                                None
                            }
                        }
                    } else {
                        // same denunciation precursor - do nothing
                        None
                    }
                }
                DenunciationStatus::DenunciationEmitted(..) => {
                    // Already 2 entries - so a Denunciation has already been created
                    None
                }
            },
            Entry::Vacant(ev) => {
                ev.insert(DenunciationStatus::Accumulating(denunciation_precursor));
                None
            }
        };

        if let Some(denunciation) = denunciation_ {
            info!("Created a new denunciation : {:?}", denunciation);
        }
        self.cleanup_caches();
    }

    /// cleanup internal cache, removing too old denunciation
    fn cleanup_caches(&mut self) {
        self.denunciations_cache.retain(|de_idx, _| {
            let slot = de_idx.get_slot();
            // Check add_denunciation_precursor notes about last_cs_final_periods.iter().min()
            !Denunciation::is_expired(
                &slot.period,
                self.last_cs_final_periods.iter().min().unwrap_or(&0),
                &self.config.denunciation_expire_periods,
            )
        });
    }

    /// get denunciations for block creation
    pub fn get_block_denunciations(&self, target_slot: &Slot) -> Vec<Denunciation> {
        let mut res = Vec::with_capacity(self.config.max_denunciations_per_block_header as usize);
        for (de_idx, de_status) in &self.denunciations_cache {
            if let DenunciationStatus::DenunciationEmitted(de) = de_status {
                // Checks
                // 1. the denunciation has not been executed already
                // 2. Denounced item slot is equal or before target slot of block header
                // 3. Denounced item slot is not too old
                let de_slot = de.get_slot();
                if !self
                    .channels
                    .execution_controller
                    .is_denunciation_executed(de_idx)
                    && de_slot <= target_slot
                    && !Denunciation::is_expired(
                        &de_slot.period,
                        &target_slot.period,
                        &self.config.denunciation_expire_periods,
                    )
                {
                    res.push(de.clone());
                }
            }

            if res.len() >= self.config.max_denunciations_per_block_header as usize {
                break;
            }
        }
        res
    }

    /// Notify of final periods
    pub(crate) fn notify_final_cs_periods(&mut self, final_cs_periods: &[u64]) {
        // update internal final CS period counter
        self.last_cs_final_periods = final_cs_periods.to_vec();

        // remove all denunciations that are expired
        self.cleanup_caches()
    }

    /// Add endorsements, turn them in DenunciationPrecursor then process
    pub(crate) fn add_endorsements(&mut self, endorsement_storage: Storage) {
        let items = endorsement_storage
            .get_endorsement_refs()
            .iter()
            .copied()
            .collect::<Vec<_>>();

        let endo_store = endorsement_storage.read_endorsements();

        for endo_id in items {
            let endo = endo_store.get(&endo_id).expect(
                "Attempting to add endorsement to denunciation pool, but it is absent from storage",
            );

            let de_p = DenunciationPrecursor::from(endo);
            self.add_denunciation_precursor(de_p);
        }
    }
}

/// A Value (as in Key/Value) for denunciation pool internal cache
#[derive(Debug)]
enum DenunciationStatus {
    /// Only 1 DenunciationPrecursor received for this key
    Accumulating(DenunciationPrecursor),
    /// 2 DenunciationPrecursor received, a Denunciation was then created
    DenunciationEmitted(Denunciation),
}
