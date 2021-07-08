#![feature(vecdeque_binary_search)]
#![feature(bool_to_option)]
#![feature(hash_drain_filter)]

#[macro_use]
extern crate logging;

mod block_graph;
mod config;
mod consensus_controller;
mod consensus_worker;
mod error;
mod misc_collections;
mod random_selector;
mod timeslots;
pub use block_graph::{
    BlockGraphExport, DiscardReason, ExportCompiledBlock, ExportDiscardedBlocks,
};
pub use config::ConsensusConfig;
pub use consensus_controller::{
    start_consensus_controller, ConsensusCommandSender, ConsensusEventReceiver, ConsensusManager,
};
pub use consensus_worker::{ConsensusCommand, ConsensusEvent};
pub use error::ConsensusError;
pub use timeslots::{
    get_block_slot_timestamp, get_current_latest_block_slot, get_latest_block_slot_at_timestamp,
    get_next_block_slot,
};

#[cfg(test)]
mod tests;
