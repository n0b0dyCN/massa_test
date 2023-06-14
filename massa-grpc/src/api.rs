// Copyright (c) 2023 MASSA LABS <info@massa.net>

use crate::error::GrpcError;
use crate::server::MassaGrpc;
use itertools::izip;
use massa_models::address::Address;
use massa_models::block::BlockGraphStatus;
use massa_models::block_id::BlockId;
use massa_models::execution::EventFilter;
use massa_models::operation::{OperationId, SecureShareOperation};
use massa_models::prehash::PreHashSet;
use massa_models::slot::Slot;
use massa_models::timeslots::{self, get_latest_block_slot_at_timestamp};
use massa_proto::massa::api::v1 as grpc;
use massa_time::MassaTime;
use std::str::FromStr;
use tracing::log::warn;

/// Default offset
const DEFAULT_OFFSET: u64 = 1;
/// Default limit
const DEFAULT_LIMIT: u64 = 50;

/// Get blocks
pub(crate) fn get_blocks(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetBlocksRequest>,
) -> Result<grpc::GetBlocksResponse, GrpcError> {
    let inner_req = request.into_inner();

    // Get the block IDs from the request.
    let blocks_ids: Vec<BlockId> = inner_req
        .queries
        .into_iter()
        .take(grpc.grpc_config.max_block_ids_per_request as usize + 1)
        .map(|query| {
            // Get the block ID from the query.
            query
                .filter
                .ok_or_else(|| GrpcError::InvalidArgument("filter is missing".to_string()))
                .and_then(|filter| {
                    BlockId::from_str(filter.id.as_str()).map_err(|_| {
                        GrpcError::InvalidArgument(format!("invalid block id: {}", filter.id))
                    })
                })
        })
        .collect::<Result<_, _>>()?;

    if blocks_ids.len() as u32 > grpc.grpc_config.max_block_ids_per_request {
        return Err(GrpcError::InvalidArgument(format!(
            "too many block ids received. Only a maximum of {} block ids are accepted per request",
            grpc.grpc_config.max_block_ids_per_request
        )));
    }

    // Get the current slot.
    let now: MassaTime = MassaTime::now()?;
    let current_slot = get_latest_block_slot_at_timestamp(
        grpc.grpc_config.thread_count,
        grpc.grpc_config.t0,
        grpc.grpc_config.genesis_timestamp,
        now,
    )?
    .unwrap_or_else(|| Slot::new(0, 0));

    // Create the context for the response.
    let context = Some(grpc::BlocksContext {
        slot: Some(current_slot.into()),
    });

    let storage = grpc.storage.clone_without_refs();
    let blocks = blocks_ids
        .into_iter()
        .filter_map(|id| {
            let content = if let Some(wrapped_block) = storage.read_blocks().get(&id) {
                wrapped_block.content.clone()
            } else {
                return None;
            };

            if let Some(graph_status) = grpc
                .consensus_controller
                .get_block_statuses(&[id])
                .into_iter()
                .next()
            {
                let mut status = Vec::new();

                if graph_status == BlockGraphStatus::Final {
                    status.push(grpc::BlockStatus::Final.into());
                };
                if graph_status == BlockGraphStatus::ActiveInBlockclique {
                    status.push(grpc::BlockStatus::InBlockclique.into());
                };
                if graph_status == BlockGraphStatus::ActiveInBlockclique
                    || graph_status == BlockGraphStatus::ActiveInAlternativeCliques
                {
                    status.push(grpc::BlockStatus::Candidate.into());
                };
                if graph_status == BlockGraphStatus::Discarded {
                    status.push(grpc::BlockStatus::Discarded.into());
                };

                return Some(grpc::BlockWrapper {
                    id: id.to_string(),
                    block: Some(content.into()),
                    status,
                });
            }

            None
        })
        .collect::<Vec<grpc::BlockWrapper>>();

    Ok(grpc::GetBlocksResponse {
        id: inner_req.id,
        context,
        blocks,
    })
}

/// get blocks by slots
pub(crate) fn get_blocks_by_slots(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetBlocksBySlotsRequest>,
) -> Result<grpc::GetBlocksBySlotsResponse, GrpcError> {
    let inner_req = request.into_inner();
    let storage = grpc.storage.clone_without_refs();

    let mut blocks = Vec::new();

    for slot in inner_req.slots.into_iter() {
        let Some(block_id) = grpc.consensus_controller.get_blockclique_block_at_slot(Slot {
            period: slot.period,
            thread: slot.thread as u8,
        }) else {
            continue;
        };

        let res = storage.read_blocks().get(&block_id).map(|b| {
            // TODO rework ?
            let header = b.clone().content.header;
            // transform to grpc struct
            let parents = header
                .content
                .parents
                .into_iter()
                .map(|p| p.to_string())
                .collect();

            let endorsements = header
                .content
                .endorsements
                .into_iter()
                .map(|endorsement| endorsement.into())
                .collect();

            let block_header = grpc::BlockHeader {
                slot: Some(grpc::Slot {
                    period: header.content.slot.period,
                    thread: header.content.slot.thread as u32,
                }),
                parents,
                operation_merkle_root: header.content.operation_merkle_root.to_string(),
                endorsements,
            };

            let operations: Vec<String> = b
                .content
                .operations
                .iter()
                .map(|ope| ope.to_string())
                .collect();

            (
                grpc::SignedBlockHeader {
                    content: Some(block_header),
                    signature: header.signature.to_string(),
                    content_creator_pub_key: header.content_creator_pub_key.to_string(),
                    content_creator_address: header.content_creator_address.to_string(),
                    id: header.id.to_string(),
                },
                operations,
            )
        });

        if let Some(block) = res {
            blocks.push(grpc::Block {
                header: Some(block.0),
                operations: block.1,
            });
        }
    }

    Ok(grpc::GetBlocksBySlotsResponse {
        id: inner_req.id,
        blocks,
    })
}

/// Get multiple datastore entries
pub(crate) fn get_datastore_entries(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetDatastoreEntriesRequest>,
) -> Result<grpc::GetDatastoreEntriesResponse, GrpcError> {
    let inner_req = request.into_inner();
    let id = inner_req.id;

    let filters = inner_req
        .queries
        .into_iter()
        .map(|query| match query.filter {
            Some(filter) => Address::from_str(filter.address.as_str())
                .map(|address| (address, filter.key))
                .map_err(|e| e.into()),
            None => Err(GrpcError::InvalidArgument("filter is missing".to_string())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let entries = grpc
        .execution_controller
        .get_final_and_active_data_entry(filters)
        .into_iter()
        .map(|output| grpc::DatastoreEntry {
            final_value: output.0.unwrap_or_default(),
            candidate_value: output.1.unwrap_or_default(),
        })
        .collect();

    Ok(grpc::GetDatastoreEntriesResponse { id, entries })
}

/// Get the largest stakers
pub(crate) fn get_largest_stakers(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetLargestStakersRequest>,
) -> Result<grpc::GetLargestStakersResponse, GrpcError> {
    let inner_req = request.into_inner();
    let id = inner_req.id;

    // Parse the query parameters, if provided.
    let query_res: Result<(u64, u64, Option<grpc::LargestStakersFilter>), GrpcError> = inner_req
        .query
        .map_or(Ok((DEFAULT_OFFSET, DEFAULT_LIMIT, None)), |query| {
            let limit = if query.limit == 0 {
                DEFAULT_LIMIT
            } else {
                query.limit
            };
            let filter = query.filter;
            // If the filter is provided, validate the minimum and maximum roll counts.
            let filter_opt = filter
                .map(|filter| {
                    if let Some(min_rolls) = filter.min_rolls {
                        if min_rolls == 0 {
                            return Err(GrpcError::InvalidArgument(
                                "min_rolls should be a positive number".into(),
                            ));
                        }
                        if let Some(max_rolls) = filter.max_rolls {
                            if max_rolls == 0 {
                                return Err(GrpcError::InvalidArgument(
                                    "max_rolls should be a positive number".into(),
                                ));
                            }
                            if min_rolls > max_rolls {
                                return Err(GrpcError::InvalidArgument(format!(
                                    "min_rolls {} cannot be greater than max_rolls {}",
                                    min_rolls, max_rolls
                                )));
                            }
                        }
                    }

                    Ok(filter)
                })
                .transpose()?; // Convert `Option<Result>` to `Result<Option>`.

            Ok((query.offset, limit, filter_opt))
        });

    let (offset, limit, filter_opt) = query_res?;

    // Get the current cycle and slot.
    let now: MassaTime = MassaTime::now()?;
    let current_slot = get_latest_block_slot_at_timestamp(
        grpc.grpc_config.thread_count,
        grpc.grpc_config.t0,
        grpc.grpc_config.genesis_timestamp,
        now,
    )?
    .unwrap_or_else(|| Slot::new(0, 0));
    let current_cycle = current_slot.get_cycle(grpc.grpc_config.periods_per_cycle);

    // Create the context for the response.
    let context = Some(grpc::LargestStakersContext {
        slot: Some(current_slot.into()),
    });

    // Get the list of stakers, filtered by the specified minimum and maximum roll counts.
    let mut staker_vec = grpc
        .execution_controller
        .get_cycle_active_rolls(current_cycle)
        .into_iter()
        .filter(|(_, rolls)| {
            filter_opt.as_ref().map_or(true, |filter| {
                if let Some(min_rolls) = filter.min_rolls {
                    if *rolls < min_rolls {
                        return false;
                    }
                }
                if let Some(max_rolls) = filter.max_rolls {
                    if *rolls > max_rolls {
                        return false;
                    }
                }
                true
            })
        })
        .map(|(address, roll_counts)| (address.to_string(), roll_counts))
        .collect::<Vec<(String, u64)>>();

    // Sort the stakers by their roll counts in descending order.
    staker_vec.sort_by_key(|&(_, roll_counts)| std::cmp::Reverse(roll_counts));

    // Paginate the stakers based on the specified offset and limit.
    let stakers = staker_vec
        .into_iter()
        .map(|(address, rolls)| grpc::LargestStakerEntry { address, rolls })
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    // Return a response with the given id, context, and the collected stakers.
    Ok(grpc::GetLargestStakersResponse {
        id,
        context,
        stakers,
    })
}

// Get node version
pub(crate) fn get_mip_status(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetMipStatusRequest>,
) -> Result<grpc::GetMipStatusResponse, GrpcError> {
    let mip_store_status_ = grpc.mip_store.get_mip_status();
    let mip_store_status: Result<Vec<grpc::MipStatusEntry>, GrpcError> = mip_store_status_
        .iter()
        .map(|(mip_info, state_id_)| {
            let state_id = grpc::ComponentStateId::from(state_id_);
            Ok(grpc::MipStatusEntry {
                mip_info: Some(grpc::MipInfo::from(mip_info)),
                state_id: i32::from(state_id),
            })
        })
        .collect();

    Ok(grpc::GetMipStatusResponse {
        id: request.into_inner().id,
        entry: mip_store_status?,
    })
}

/// Get next block best parents
pub(crate) fn get_next_block_best_parents(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetNextBlockBestParentsRequest>,
) -> Result<grpc::GetNextBlockBestParentsResponse, GrpcError> {
    let inner_req = request.into_inner();
    let parents = grpc
        .consensus_controller
        .get_best_parents()
        .into_iter()
        .map(|p| grpc::BlockParent {
            block_id: p.0.to_string(),
            period: p.1,
        })
        .collect();
    Ok(grpc::GetNextBlockBestParentsResponse {
        id: inner_req.id,
        parents,
    })
}

/// Get operations
pub(crate) fn get_operations(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetOperationsRequest>,
) -> Result<grpc::GetOperationsResponse, GrpcError> {
    let storage = grpc.storage.clone_without_refs();
    let inner_req: grpc::GetOperationsRequest = request.into_inner();
    let id = inner_req.id;

    let operations_ids: Vec<OperationId> = inner_req
        .queries
        .into_iter()
        .take(grpc.grpc_config.max_operation_ids_per_request as usize + 1)
        .map(|query| {
            query
                .filter
                .ok_or_else(|| GrpcError::InvalidArgument("filter is missing".to_string()))
                .and_then(|filter| {
                    OperationId::from_str(filter.id.as_str()).map_err(|_| {
                        GrpcError::InvalidArgument(format!("invalid operation id: {}", filter.id))
                    })
                })
        })
        .collect::<Result<_, _>>()?;

    if operations_ids.len() as u32 > grpc.grpc_config.max_operation_ids_per_request {
        return Err(GrpcError::InvalidArgument(format!("too many operations received. Only a maximum of {} operations are accepted per request", grpc.grpc_config.max_operation_ids_per_request)));
    }

    // Get the current slot.
    let now: MassaTime = MassaTime::now()?;
    let current_slot = get_latest_block_slot_at_timestamp(
        grpc.grpc_config.thread_count,
        grpc.grpc_config.t0,
        grpc.grpc_config.genesis_timestamp,
        now,
    )?
    .unwrap_or_else(|| Slot::new(0, 0));

    // Create the context for the response.
    let context = Some(grpc::OperationsContext {
        slot: Some(current_slot.into()),
    });

    // Get the operations and the list of blocks that contain them from storage
    let storage_info: Vec<(SecureShareOperation, PreHashSet<BlockId>)> = {
        let read_blocks = storage.read_blocks();
        let read_ops = storage.read_operations();
        operations_ids
            .iter()
            .filter_map(|id| {
                read_ops.get(id).cloned().map(|op| {
                    (
                        op,
                        read_blocks
                            .get_blocks_by_operation(id)
                            .cloned()
                            .unwrap_or_default(),
                    )
                })
            })
            .collect()
    };

    // Keep only the ops id (found in storage)
    let ops: Vec<OperationId> = storage_info.iter().map(|(op, _)| op.id).collect();

    // Get the speculative and final execution status of the operations
    let exec_statuses: Vec<_> = grpc
        .execution_controller
        .get_ops_exec_status(&ops)
        .into_iter()
        .map(|(spec_exec, final_exex)| {
            let mut grpc_statuses: Vec<_> = Vec::new();
            match (spec_exec, final_exec) {
                (Some(true), Some(true)) => {
                    status.push(grpc::OperationStatus::Success.into());
                    status.push(grpc::OperationStatus::Final.into())
                }
                (Some(false), Some(false)) => {
                    status.push(grpc::OperationStatus::Failure.into());
                    status.push(grpc::OperationStatus::Final.into())
                }
                (Some(true), None) => {
                    status.push(grpc::OperationStatus::Success.into());
                    status.push(grpc::OperationStatus::Pending.into())
                }
                (Some(false), None) => {
                    status.push(grpc::OperationStatus::Failure.into());
                    status.push(grpc::OperationStatus::Pending.into())
                }
                _ => {
                    status.push(grpc::OperationStatus::Unknown.into());
                }
            }
            grpc_statuses
        })
        .collect();

    // Gather all values into a vector of OperationWrapper instances
    let mut operations: Vec<grpc::OperationWrapper> = Vec::with_capacity(ops.len());
    let zipped_iterator = izip!(
        ops.into_iter(),
        storage_info.into_iter(),
        exec_statuses.into_iter(),
    );
    for (id, (operation, in_blocks), mut exec_status) in zipped_iterator {
        operations.push(grpc::OperationWrapper {
            id: id.to_string(),
            thread: operation
                .content_creator_address
                .get_thread(grpc.grpc_config.thread_count) as u32,
            operation: Some(operation.into()),
            block_ids: in_blocks.into_iter().map(|id| id.to_string()).collect(),
            status: exec_status,
        });
    }

    Ok(grpc::GetOperationsResponse {
        id,
        context,
        operations,
    })
}

/// Get smart contract execution events
pub(crate) fn get_sc_execution_events(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetScExecutionEventsRequest>,
) -> Result<grpc::GetScExecutionEventsResponse, GrpcError> {
    let inner_req: grpc::GetScExecutionEventsRequest = request.into_inner();
    let id = inner_req.id;

    let event_filter = inner_req
        .query
        .map_or(Ok(EventFilter::default()), |query| {
            query.filter.map_or(Ok(EventFilter::default()), |filter| {
                filter.try_into().map_err(|e| {
                    GrpcError::InvalidArgument(format!("failed to parse filter due to: {}", e))
                })
            })
        })?;

    // Get the current slot.
    let now: MassaTime = MassaTime::now()?;
    let current_slot = get_latest_block_slot_at_timestamp(
        grpc.grpc_config.thread_count,
        grpc.grpc_config.t0,
        grpc.grpc_config.genesis_timestamp,
        now,
    )?
    .unwrap_or_else(|| Slot::new(0, 0));

    // Create the context for the response.
    let context = Some(grpc::GetScExecutionEventsContext {
        slot: Some(current_slot.into()),
    });

    let events: Vec<grpc::ScExecutionEvent> = grpc
        .execution_controller
        .get_filtered_sc_output_event(event_filter)
        .into_iter()
        .map(|event| event.into())
        .collect();

    Ok(grpc::GetScExecutionEventsResponse {
        id,
        context,
        events,
    })
}

//  Get selector draws
pub(crate) fn get_selector_draws(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetSelectorDrawsRequest>,
) -> Result<grpc::GetSelectorDrawsResponse, GrpcError> {
    let inner_req = request.into_inner();
    let id = inner_req.id;

    let addresses = inner_req
        .queries
        .into_iter()
        .map(|query| match query.filter {
            Some(filter) => Address::from_str(filter.address.as_str()).map_err(|e| e.into()),
            None => Err(GrpcError::InvalidArgument("filter is missing".to_string())),
        })
        .collect::<Result<Vec<_>, _>>()?;

    // get future draws from selector
    let selection_draws = {
        let cur_slot = match timeslots::get_current_latest_block_slot(
            grpc.grpc_config.thread_count,
            grpc.grpc_config.t0,
            grpc.grpc_config.genesis_timestamp,
        ) {
            Ok(slot) => slot.unwrap_or_else(Slot::min),
            Err(e) => {
                warn!("failed to get current slot with error: {}", e);
                Slot::min()
            }
        };

        let slot_end = Slot::new(
            cur_slot
                .period
                .saturating_add(grpc.grpc_config.draw_lookahead_period_count),
            cur_slot.thread,
        );
        addresses
            .iter()
            .map(|addr| {
                let (nt_block_draws, nt_endorsement_draws) = grpc
                    .selector_controller
                    .get_address_selections(addr, cur_slot, slot_end)
                    .unwrap_or_default();

                let mut proto_nt_block_draws = Vec::with_capacity(addresses.len());
                let mut proto_nt_endorsement_draws = Vec::with_capacity(addresses.len());
                let iterator = izip!(nt_block_draws.into_iter(), nt_endorsement_draws.into_iter());
                for (next_block_draw, next_endorsement_draw) in iterator {
                    proto_nt_block_draws.push(next_block_draw.into());
                    proto_nt_endorsement_draws.push(next_endorsement_draw.into());
                }

                (proto_nt_block_draws, proto_nt_endorsement_draws)
            })
            .collect::<Vec<_>>()
    };

    // Compile results
    let mut res = Vec::with_capacity(addresses.len());
    let iterator = izip!(addresses.into_iter(), selection_draws.into_iter());
    for (address, (next_block_draws, next_endorsement_draws)) in iterator {
        res.push(grpc::SelectorDraws {
            address: address.to_string(),
            next_block_draws,
            next_endorsement_draws,
        });
    }

    Ok(grpc::GetSelectorDrawsResponse {
        id,
        selector_draws: res,
    })
}

/// Get transactions throughput
pub(crate) fn get_transactions_throughput(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetTransactionsThroughputRequest>,
) -> Result<grpc::GetTransactionsThroughputResponse, GrpcError> {
    let stats = grpc.execution_controller.get_stats();
    let nb_sec_range = stats
        .time_window_end
        .saturating_sub(stats.time_window_start)
        .to_duration()
        .as_secs();

    // checked_div
    let throughput = stats
        .final_executed_operations_count
        .checked_div(nb_sec_range as usize)
        .unwrap_or_default() as u32;

    Ok(grpc::GetTransactionsThroughputResponse {
        id: request.into_inner().id,
        throughput,
    })
}

// Get node version
pub(crate) fn get_version(
    grpc: &MassaGrpc,
    request: tonic::Request<grpc::GetVersionRequest>,
) -> Result<grpc::GetVersionResponse, GrpcError> {
    Ok(grpc::GetVersionResponse {
        id: request.into_inner().id,
        version: grpc.version.to_string(),
    })
}
