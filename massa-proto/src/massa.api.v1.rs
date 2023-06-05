/// When an address is drawn to create an endorsement it is selected for a specific index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedSlot {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index in the slot
    #[prost(fixed64, tag = "2")]
    pub index: u64,
}
/// A point in time where a block is expected
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Slot {
    /// Period
    #[prost(fixed64, tag = "1")]
    pub period: u64,
    /// Thread
    #[prost(fixed32, tag = "2")]
    pub thread: u32,
}
/// An endorsement, as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endorsement {
    /// Slot in which the endorsement can be included
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Endorsement index inside the including block
    #[prost(fixed32, tag = "2")]
    pub index: u32,
    /// Hash of endorsed block
    /// This is the parent in thread `self.slot.thread` of the block in which the endorsement is included
    #[prost(string, tag = "3")]
    pub endorsed_block: ::prost::alloc::string::String,
}
/// Signed endorsement
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedEndorsement {
    /// Endorsement
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Endorsement>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// BytesMapFieldEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMapFieldEntry {
    /// bytes key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// bytes key
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Packages a type such that it can be securely sent and received in a trust-free network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecureShare {
    /// Content in sharable, deserializable form. Is used in the secure verification protocols
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_data: ::prost::alloc::vec::Vec<u8>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// The operation as sent in the network
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The fee they have decided for this operation
    #[prost(fixed64, tag = "1")]
    pub fee: u64,
    /// After `expire_period` slot the operation won't be included in a block
    #[prost(fixed64, tag = "2")]
    pub expire_period: u64,
    /// The type specific operation part
    #[prost(message, optional, tag = "3")]
    pub op: ::core::option::Option<OperationType>,
}
/// Type specific operation content
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationType {
    /// Transfer coins from sender to recipient
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<Transaction>,
    /// The sender buys `roll_count` rolls. Roll price is defined in configuration
    #[prost(message, optional, tag = "2")]
    pub roll_buy: ::core::option::Option<RollBuy>,
    /// The sender sells `roll_count` rolls. Roll price is defined in configuration
    #[prost(message, optional, tag = "3")]
    pub roll_sell: ::core::option::Option<RollSell>,
    /// Execute a smart contract
    #[prost(message, optional, tag = "4")]
    pub execut_sc: ::core::option::Option<ExecuteSc>,
    /// Calls an exported function from a stored smart contract
    #[prost(message, optional, tag = "5")]
    pub call_sc: ::core::option::Option<CallSc>,
}
/// Transfer coins from sender to recipient
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// Recipient address
    #[prost(string, tag = "1")]
    pub recipient_address: ::prost::alloc::string::String,
    /// Amount
    #[prost(fixed64, tag = "2")]
    pub amount: u64,
}
/// The sender buys `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollBuy {
    /// Roll count
    #[prost(fixed64, tag = "1")]
    pub roll_count: u64,
}
/// The sender sells `roll_count` rolls. Roll price is defined in configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollSell {
    /// Roll count
    #[prost(fixed64, tag = "1")]
    pub roll_count: u64,
}
/// Execute a smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteSc {
    /// Smart contract bytecode.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// The maximum of coins that could be spent by the operation sender
    #[prost(fixed64, tag = "2")]
    pub max_coins: u64,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(fixed64, tag = "3")]
    pub max_gas: u64,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "4")]
    pub datastore: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Calls an exported function from a stored smart contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallSc {
    /// Target smart contract address
    #[prost(string, tag = "1")]
    pub target_addr: ::prost::alloc::string::String,
    /// Target function name. No function is called if empty
    #[prost(string, tag = "2")]
    pub target_func: ::prost::alloc::string::String,
    /// Parameter to pass to the target function
    #[prost(bytes = "vec", tag = "3")]
    pub param: ::prost::alloc::vec::Vec<u8>,
    /// The maximum amount of gas that the execution of the contract is allowed to cost
    #[prost(fixed64, tag = "4")]
    pub max_gas: u64,
    /// Extra coins that are spent from the caller's balance and transferred to the target
    #[prost(fixed64, tag = "5")]
    pub coins: u64,
}
/// Signed operation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedOperation {
    /// Operation
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Operation>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// A wrapper around an operation with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationWrapper {
    /// The unique ID of the operation.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The IDs of the blocks in which the operation appears
    #[prost(string, repeated, tag = "3")]
    pub block_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The thread in which the operation can be included
    #[prost(fixed32, tag = "5")]
    pub thread: u32,
    /// The operation object itself
    #[prost(message, optional, tag = "6")]
    pub operation: ::core::option::Option<SignedOperation>,
    /// The execution statuses of the operation
    #[prost(enumeration = "OperationStatus", repeated, tag = "7")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for an operation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationStatus {
    /// Default enum value
    Unspecified = 0,
    /// The operation is still pending
    Pending = 1,
    /// The operation is final
    Final = 2,
    /// The operation was executed successfully
    Success = 3,
    /// The operation failed to execute
    Failure = 4,
    /// The status of the operation is unknown
    Unknown = 5,
}
impl OperationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationStatus::Unspecified => "OPERATION_STATUS_UNSPECIFIED",
            OperationStatus::Pending => "OPERATION_STATUS_PENDING",
            OperationStatus::Final => "OPERATION_STATUS_FINAL",
            OperationStatus::Success => "OPERATION_STATUS_SUCCESS",
            OperationStatus::Failure => "OPERATION_STATUS_FAILURE",
            OperationStatus::Unknown => "OPERATION_STATUS_UNKNOWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATUS_PENDING" => Some(Self::Pending),
            "OPERATION_STATUS_FINAL" => Some(Self::Final),
            "OPERATION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_STATUS_FAILURE" => Some(Self::Failure),
            "OPERATION_STATUS_UNKNOWN" => Some(Self::Unknown),
            _ => None,
        }
    }
}
/// Block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations ids
    #[prost(string, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Filled block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledBlock {
    /// Signed header
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<SignedBlockHeader>,
    /// Operations
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<FilledOperationTuple>,
}
/// Block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// parents
    #[prost(string, repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// All operations hash
    #[prost(string, tag = "3")]
    pub operation_merkle_root: ::prost::alloc::string::String,
    /// Signed endorsements
    #[prost(message, repeated, tag = "4")]
    pub endorsements: ::prost::alloc::vec::Vec<SignedEndorsement>,
}
/// Filled Operation Tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilledOperationTuple {
    /// Operation id
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// Signed operation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<SignedOperation>,
}
/// Signed block
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlock {
    /// Block
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Block>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// Signed block header
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedBlockHeader {
    /// BlockHeader
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<BlockHeader>,
    /// A cryptographically generated value using `serialized_data` and a public key.
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// The public-key component used in the generation of the signature
    #[prost(string, tag = "3")]
    pub content_creator_pub_key: ::prost::alloc::string::String,
    /// Derived from the same public key used to generate the signature
    #[prost(string, tag = "4")]
    pub content_creator_address: ::prost::alloc::string::String,
    /// A secure hash of the data. See also \[massa_hash::Hash\]
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
}
/// A wrapper around a block with its metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockWrapper {
    /// The unique ID of the block.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The block object itself
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    /// The execution statuses of the block
    #[prost(enumeration = "BlockStatus", repeated, tag = "3")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// Possible statuses for a block
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockStatus {
    /// Default enum value
    Unspecified = 0,
    /// The block is in the greatest clique (and not final)
    InBlockclique = 1,
    /// The block is final
    Final = 2,
    /// The block is candidate (active any clique but not final)
    Candidate = 3,
    /// The block is discarded
    Discarded = 4,
}
impl BlockStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BlockStatus::Unspecified => "BLOCK_STATUS_UNSPECIFIED",
            BlockStatus::InBlockclique => "BLOCK_STATUS_IN_BLOCKCLIQUE",
            BlockStatus::Final => "BLOCK_STATUS_FINAL",
            BlockStatus::Candidate => "BLOCK_STATUS_CANDIDATE",
            BlockStatus::Discarded => "BLOCK_STATUS_DISCARDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BLOCK_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BLOCK_STATUS_IN_BLOCKCLIQUE" => Some(Self::InBlockclique),
            "BLOCK_STATUS_FINAL" => Some(Self::Final),
            "BLOCK_STATUS_CANDIDATE" => Some(Self::Candidate),
            "BLOCK_STATUS_DISCARDED" => Some(Self::Discarded),
            _ => None,
        }
    }
}
/// SlotExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotExecutionOutput {
    /// Status
    #[prost(enumeration = "ExecutionOutputStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
    /// Executed slot output
    #[prost(message, optional, tag = "2")]
    pub execution_output: ::core::option::Option<ExecutionOutput>,
}
/// FinalizedExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizedExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// ExecutionOutput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionOutput {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Block id at that slot (optional)
    #[prost(string, optional, tag = "2")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Events emitted by the execution step
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<ScExecutionEvent>,
    /// State changes caused by the execution step
    #[prost(message, optional, tag = "4")]
    pub state_changes: ::core::option::Option<StateChanges>,
}
/// ScExecutionEvent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEvent {
    /// Sc execution context
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<ScExecutionEventContext>,
    /// json data string
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// ScExecutionEvent context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScExecutionEventContext {
    /// base58 encoded slot(period + thread) + index_in_slot
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// When was it generated
    #[prost(message, optional, tag = "2")]
    pub origin_slot: ::core::option::Option<Slot>,
    /// Block id if there was a block at that slot (optional)
    #[prost(string, optional, tag = "3")]
    pub block_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Index of the event in the slot
    #[prost(fixed64, tag = "4")]
    pub index_in_slot: u64,
    /// Call stack addresses. most recent at the end
    #[prost(string, repeated, tag = "5")]
    pub call_stack: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Origin operation id (optional)
    #[prost(string, optional, tag = "6")]
    pub origin_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "7")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// StateChanges
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateChanges {
    /// Ledger changes
    #[prost(message, repeated, tag = "1")]
    pub ledger_changes: ::prost::alloc::vec::Vec<LedgerChangeEntry>,
    /// Asynchronous pool changes
    #[prost(message, repeated, tag = "2")]
    pub async_pool_changes: ::prost::alloc::vec::Vec<AsyncPoolChangeEntry>,
    /// Executed operations changes
    #[prost(message, repeated, tag = "4")]
    pub executed_ops_changes: ::prost::alloc::vec::Vec<ExecutedOpsChangeEntry>,
    /// Executed denunciations changes
    #[prost(message, repeated, tag = "5")]
    pub executed_denunciations_changes: ::prost::alloc::vec::Vec<DenunciationIndex>,
}
/// ExecutedOpsChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeEntry {
    /// OperationId
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    /// ExecutedOpsChangeValue
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ExecutedOpsChangeValue>,
}
/// ExecutedOpsChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutedOpsChangeValue {
    /// The status of the execution of the operation
    #[prost(enumeration = "OperationExecutionStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
    /// Slot until which the operation remains valid (included)
    #[prost(message, optional, tag = "2")]
    pub slot: ::core::option::Option<Slot>,
}
/// AsyncPoolChange Entry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeEntry {
    /// Async message id
    #[prost(string, tag = "1")]
    pub async_message_id: ::prost::alloc::string::String,
    /// AsyncPool message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AsyncPoolChangeValue>,
}
/// AsyncPoolChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncPoolChangeValue {
    /// The type of the change
    #[prost(enumeration = "AsyncPoolChangeType", tag = "1")]
    pub r#type: i32,
    /// AsyncPool message
    #[prost(oneof = "async_pool_change_value::Message", tags = "2, 3")]
    pub message: ::core::option::Option<async_pool_change_value::Message>,
}
/// Nested message and enum types in `AsyncPoolChangeValue`.
pub mod async_pool_change_value {
    /// AsyncPool message
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Created ledger entry
        #[prost(message, tag = "2")]
        CreatedMessage(super::AsyncMessage),
        /// Update ledger entry
        #[prost(message, tag = "3")]
        UpdatedMessage(super::AsyncMessageUpdate),
    }
}
/// Asynchronous smart contract message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessage {
    /// Slot at which the message was emitted
    #[prost(message, optional, tag = "1")]
    pub emission_slot: ::core::option::Option<Slot>,
    /// Index of the emitted message within the `emission_slot`.
    /// This is used for disambiguate the emission of multiple messages at the same slot.
    #[prost(fixed64, tag = "2")]
    pub emission_index: u64,
    /// The address that sent the message
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// The address towards which the message is being sent
    #[prost(string, tag = "4")]
    pub destination: ::prost::alloc::string::String,
    /// the handler function name within the destination address' bytecode
    #[prost(string, tag = "5")]
    pub handler: ::prost::alloc::string::String,
    /// Maximum gas to use when processing the message
    #[prost(fixed64, tag = "6")]
    pub max_gas: u64,
    /// Fee paid by the sender when the message is processed.
    #[prost(fixed64, tag = "7")]
    pub fee: u64,
    /// Coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(fixed64, tag = "8")]
    pub coins: u64,
    /// Slot at which the message starts being valid (bound included in the validity range)
    #[prost(message, optional, tag = "9")]
    pub validity_start: ::core::option::Option<Slot>,
    /// Slot at which the message stops being valid (bound not included in the validity range)
    #[prost(message, optional, tag = "10")]
    pub validity_end: ::core::option::Option<Slot>,
    /// Raw payload data of the message
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Trigger that define whenever a message can be executed
    #[prost(message, optional, tag = "12")]
    pub trigger: ::core::option::Option<AsyncMessageTrigger>,
    /// Boolean that determine if the message can be executed. For messages without filter this boolean is always true.
    /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
    #[prost(bool, tag = "13")]
    pub can_be_executed: bool,
    /// Hash of the message
    #[prost(string, tag = "14")]
    pub hash: ::prost::alloc::string::String,
}
/// Asynchronous smart contract message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageUpdate {
    /// Change the slot at which the message was emitted
    #[prost(message, optional, tag = "1")]
    pub emission_slot: ::core::option::Option<SetOrKeepSlot>,
    /// Change the index of the emitted message within the `emission_slot`.
    /// This is used for disambiguate the emission of multiple messages at the same slot.
    #[prost(message, optional, tag = "2")]
    pub emission_index: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the address that sent the message
    #[prost(message, optional, tag = "3")]
    pub sender: ::core::option::Option<SetOrKeepString>,
    /// Change the address towards which the message is being sent
    #[prost(message, optional, tag = "4")]
    pub destination: ::core::option::Option<SetOrKeepString>,
    /// Change the handler function name within the destination address' bytecode
    #[prost(message, optional, tag = "5")]
    pub handler: ::core::option::Option<SetOrKeepString>,
    /// Change the maximum gas to use when processing the message
    #[prost(message, optional, tag = "6")]
    pub max_gas: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the fee paid by the sender when the message is processed.
    #[prost(message, optional, tag = "7")]
    pub fee: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the coins sent from the sender to the target address of the message.
    /// Those coins are spent by the sender address when the message is sent,
    /// and credited to the destination address when receiving the message.
    /// In case of failure or discard, those coins are reimbursed to the sender.
    #[prost(message, optional, tag = "8")]
    pub coins: ::core::option::Option<SetOrKeepFixed64>,
    /// Change the slot at which the message starts being valid (bound included in the validity range)
    #[prost(message, optional, tag = "9")]
    pub validity_start: ::core::option::Option<SetOrKeepSlot>,
    /// Change the slot at which the message stops being valid (bound not included in the validity range)
    #[prost(message, optional, tag = "10")]
    pub validity_end: ::core::option::Option<SetOrKeepSlot>,
    /// Change the raw payload data of the message
    #[prost(message, optional, tag = "11")]
    pub data: ::core::option::Option<SetOrKeepBytes>,
    /// Change the trigger that define whenever a message can be executed
    #[prost(message, optional, tag = "12")]
    pub trigger: ::core::option::Option<SetOrKeepAsyncMessageTrigger>,
    /// Change the boolean that determine if the message can be executed. For messages without filter this boolean is always true.
    /// For messages with filter, this boolean is true if the filter has been matched between `validity_start` and current slot.
    #[prost(message, optional, tag = "13")]
    pub can_be_executed: ::core::option::Option<SetOrKeepBool>,
    /// Change the hash of the message
    #[prost(message, optional, tag = "14")]
    pub hash: ::core::option::Option<SetOrKeepString>,
}
/// Set or Keep Slot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepSlot {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Slot>,
}
/// Set or Keep Fixed64
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepFixed64 {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(fixed64, optional, tag = "2")]
    pub value: ::core::option::Option<u64>,
}
/// Set or Keep String
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepString {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Set or Keep Bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytes {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Set or Keep Bool
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBool {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(bool, optional, tag = "2")]
    pub value: ::core::option::Option<bool>,
}
/// Set or Keep AsyncMessageTrigger
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepAsyncMessageTrigger {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The value of that entry (optional)
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<AsyncMessageTrigger>,
}
/// Structure defining a trigger for an asynchronous message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncMessageTrigger {
    /// Filter on the address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Filter on the datastore key (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub datastore_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// LedgerChangeEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Ledger message
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<LedgerChangeValue>,
}
/// LedgerChangeValue
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerChangeValue {
    /// The type of the change
    #[prost(enumeration = "LedgerChangeType", tag = "1")]
    pub r#type: i32,
    /// LedgerEntry or LedgerEntryUpdate
    #[prost(oneof = "ledger_change_value::Entry", tags = "2, 3")]
    pub entry: ::core::option::Option<ledger_change_value::Entry>,
}
/// Nested message and enum types in `LedgerChangeValue`.
pub mod ledger_change_value {
    /// LedgerEntry or LedgerEntryUpdate
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Created ledger entry
        #[prost(message, tag = "2")]
        CreatedEntry(super::LedgerEntry),
        /// Update ledger entry
        #[prost(message, tag = "3")]
        UpdatedEntry(super::LedgerEntryUpdate),
    }
}
/// An entry associated to an address in the `FinalLedger`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntry {
    /// The balance of that entry
    #[prost(fixed64, tag = "1")]
    pub balance: u64,
    /// Executable bytecode
    #[prost(bytes = "vec", tag = "2")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    /// A key-value store associating a hash to arbitrary bytes
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<BytesMapFieldEntry>,
}
/// Represents an update to one or more fields of a `LedgerEntry`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LedgerEntryUpdate {
    /// Change the balance
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<SetOrKeepBalance>,
    /// Change the executable bytecode
    #[prost(message, optional, tag = "2")]
    pub bytecode: ::core::option::Option<SetOrKeepBytecode>,
    /// / Change datastore entries
    #[prost(message, repeated, tag = "3")]
    pub datastore: ::prost::alloc::vec::Vec<SetOrDeleteDatastoreEntry>,
}
/// Set or Keep Balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBalance {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optional)
    #[prost(fixed64, optional, tag = "2")]
    pub balance: ::core::option::Option<u64>,
}
/// Set or Keep Bytecode
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrKeepBytecode {
    /// The type of the change
    #[prost(enumeration = "SetOrKeepType", tag = "1")]
    pub r#type: i32,
    /// Executable bytecode (optional)
    #[prost(bytes = "vec", optional, tag = "2")]
    pub bytecode: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Set or Delete DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOrDeleteDatastoreEntry {
    /// The type of the change
    #[prost(enumeration = "SetOrDeleteType", tag = "1")]
    pub r#type: i32,
    /// The balance of that entry (optioal)
    #[prost(message, optional, tag = "2")]
    pub datastore_entry: ::core::option::Option<BytesMapFieldEntry>,
}
/// Index for Denunciations in collections (e.g. like a HashMap...)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationIndex {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[prost(oneof = "denunciation_index::Entry", tags = "1, 2")]
    pub entry: ::core::option::Option<denunciation_index::Entry>,
}
/// Nested message and enum types in `DenunciationIndex`.
pub mod denunciation_index {
    /// DenunciationBlockHeader or DenunciationEndorsement
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Entry {
        /// Denunciation block header
        #[prost(message, tag = "1")]
        BlockHeader(super::DenunciationBlockHeader),
        /// Denunciation endorsement
        #[prost(message, tag = "2")]
        Endorsement(super::DenunciationEndorsement),
    }
}
/// Variant for Block header denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationBlockHeader {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// Variant for Endorsement denunciation index
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenunciationEndorsement {
    /// Denounciation slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
    /// Denounciation index
    #[prost(fixed32, tag = "2")]
    pub index: u32,
}
/// ScExecutionEventStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScExecutionEventStatus {
    /// Default enum value
    Unspecified = 0,
    /// Final status
    Final = 1,
    /// Read only status
    ReadOnly = 2,
    /// Failure status
    Failure = 3,
}
impl ScExecutionEventStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScExecutionEventStatus::Unspecified => {
                "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED"
            }
            ScExecutionEventStatus::Final => "SC_EXECUTION_EVENT_STATUS_FINAL",
            ScExecutionEventStatus::ReadOnly => "SC_EXECUTION_EVENT_STATUS_READ_ONLY",
            ScExecutionEventStatus::Failure => "SC_EXECUTION_EVENT_STATUS_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SC_EXECUTION_EVENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SC_EXECUTION_EVENT_STATUS_FINAL" => Some(Self::Final),
            "SC_EXECUTION_EVENT_STATUS_READ_ONLY" => Some(Self::ReadOnly),
            "SC_EXECUTION_EVENT_STATUS_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// ExecutionOutputStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionOutputStatus {
    /// Default enum value
    Unspecified = 0,
    /// Candidate status
    Candidate = 1,
    /// Final status
    Final = 2,
}
impl ExecutionOutputStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionOutputStatus::Unspecified => "EXECUTION_OUTPUT_STATUS_UNSPECIFIED",
            ExecutionOutputStatus::Candidate => "EXECUTION_OUTPUT_STATUS_CANDIDATE",
            ExecutionOutputStatus::Final => "EXECUTION_OUTPUT_STATUS_FINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_OUTPUT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXECUTION_OUTPUT_STATUS_CANDIDATE" => Some(Self::Candidate),
            "EXECUTION_OUTPUT_STATUS_FINAL" => Some(Self::Final),
            _ => None,
        }
    }
}
/// OperationExecutionStatus type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationExecutionStatus {
    /// Default enum value
    Unspecified = 0,
    /// Success status
    Success = 1,
    /// Failed only status
    Failed = 2,
}
impl OperationExecutionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationExecutionStatus::Unspecified => {
                "OPERATION_EXECUTION_STATUS_UNSPECIFIED"
            }
            OperationExecutionStatus::Success => "OPERATION_EXECUTION_STATUS_SUCCESS",
            OperationExecutionStatus::Failed => "OPERATION_EXECUTION_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_EXECUTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_EXECUTION_STATUS_SUCCESS" => Some(Self::Success),
            "OPERATION_EXECUTION_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// AsyncPoolChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AsyncPoolChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Add type
    Set = 1,
    /// Activate only type
    Update = 2,
    /// Delete only type
    Delete = 3,
}
impl AsyncPoolChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AsyncPoolChangeType::Unspecified => "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED",
            AsyncPoolChangeType::Set => "ASYNC_POOL_CHANGE_TYPE_SET",
            AsyncPoolChangeType::Update => "ASYNC_POOL_CHANGE_TYPE_UPDATE",
            AsyncPoolChangeType::Delete => "ASYNC_POOL_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASYNC_POOL_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASYNC_POOL_CHANGE_TYPE_SET" => Some(Self::Set),
            "ASYNC_POOL_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "ASYNC_POOL_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// LedgerChangeType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LedgerChangeType {
    /// Default enum value
    Unspecified = 0,
    /// Set type
    Set = 1,
    /// Update type
    Update = 2,
    /// Delete type
    Delete = 3,
}
impl LedgerChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LedgerChangeType::Unspecified => "LEDGER_CHANGE_TYPE_UNSPECIFIED",
            LedgerChangeType::Set => "LEDGER_CHANGE_TYPE_SET",
            LedgerChangeType::Update => "LEDGER_CHANGE_TYPE_UPDATE",
            LedgerChangeType::Delete => "LEDGER_CHANGE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LEDGER_CHANGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LEDGER_CHANGE_TYPE_SET" => Some(Self::Set),
            "LEDGER_CHANGE_TYPE_UPDATE" => Some(Self::Update),
            "LEDGER_CHANGE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// SetOrKeepType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrKeepType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Keeps the existing value
    Keep = 2,
}
impl SetOrKeepType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrKeepType::Unspecified => "SET_OR_KEEP_TYPE_UNSPECIFIED",
            SetOrKeepType::Set => "SET_OR_KEEP_TYPE_SET",
            SetOrKeepType::Keep => "SET_OR_KEEP_TYPE_KEEP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_KEEP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_KEEP_TYPE_SET" => Some(Self::Set),
            "SET_OR_KEEP_TYPE_KEEP" => Some(Self::Keep),
            _ => None,
        }
    }
}
/// SetOrDeleteType type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SetOrDeleteType {
    /// Default enum value
    Unspecified = 0,
    /// Sets a new absolute value
    Set = 1,
    /// Deletes the existing value
    Delete = 2,
}
impl SetOrDeleteType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SetOrDeleteType::Unspecified => "SET_OR_DELETE_TYPE_UNSPECIFIED",
            SetOrDeleteType::Set => "SET_OR_DELETE_TYPE_SET",
            SetOrDeleteType::Delete => "SET_OR_DELETE_TYPE_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SET_OR_DELETE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SET_OR_DELETE_TYPE_SET" => Some(Self::Set),
            "SET_OR_DELETE_TYPE_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// Entry for GetMipStatusResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipStatusEntry {
    /// Mip info
    #[prost(message, optional, tag = "1")]
    pub mip_info: ::core::option::Option<MipInfo>,
    /// state id
    #[prost(enumeration = "ComponentStateId", tag = "2")]
    pub state_id: i32,
}
/// Same as MipInfo struct in versioning package
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(fixed32, tag = "2")]
    pub version: u32,
    #[prost(fixed64, tag = "3")]
    pub start: u64,
    #[prost(fixed64, tag = "4")]
    pub timeout: u64,
    #[prost(fixed64, tag = "5")]
    pub activation_delay: u64,
    #[prost(message, repeated, tag = "6")]
    pub components: ::prost::alloc::vec::Vec<MipComponentEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MipComponentEntry {
    #[prost(enumeration = "MipComponent", tag = "1")]
    pub kind: i32,
    #[prost(fixed32, tag = "2")]
    pub version: u32,
}
/// Same as ComponentStateId enum in versioning package
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComponentStateId {
    Unspecified = 0,
    Error = 1,
    Defined = 2,
    Started = 3,
    Lockedin = 4,
    Active = 5,
    Failed = 6,
}
impl ComponentStateId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComponentStateId::Unspecified => "COMPONENT_STATE_ID_UNSPECIFIED",
            ComponentStateId::Error => "COMPONENT_STATE_ID_ERROR",
            ComponentStateId::Defined => "COMPONENT_STATE_ID_DEFINED",
            ComponentStateId::Started => "COMPONENT_STATE_ID_STARTED",
            ComponentStateId::Lockedin => "COMPONENT_STATE_ID_LOCKEDIN",
            ComponentStateId::Active => "COMPONENT_STATE_ID_ACTIVE",
            ComponentStateId::Failed => "COMPONENT_STATE_ID_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPONENT_STATE_ID_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPONENT_STATE_ID_ERROR" => Some(Self::Error),
            "COMPONENT_STATE_ID_DEFINED" => Some(Self::Defined),
            "COMPONENT_STATE_ID_STARTED" => Some(Self::Started),
            "COMPONENT_STATE_ID_LOCKEDIN" => Some(Self::Lockedin),
            "COMPONENT_STATE_ID_ACTIVE" => Some(Self::Active),
            "COMPONENT_STATE_ID_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
/// Versioning component enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MipComponent {
    Unspecified = 0,
    Address = 1,
    Keypair = 2,
}
impl MipComponent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MipComponent::Unspecified => "MIP_COMPONENT_UNSPECIFIED",
            MipComponent::Address => "MIP_COMPONENT_ADDRESS",
            MipComponent::Keypair => "MIP_COMPONENT_KEYPAIR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MIP_COMPONENT_UNSPECIFIED" => Some(Self::Unspecified),
            "MIP_COMPONENT_ADDRESS" => Some(Self::Address),
            "MIP_COMPONENT_KEYPAIR" => Some(Self::Keypair),
            _ => None,
        }
    }
}
/// GetBlocksRequest holds request for GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<GetBlocksQuery>,
}
/// GetBlocks Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetBlocksFilter>,
}
/// GetBlocks Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksFilter {
    /// Block id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetBlocksResponse holds response from GetBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<BlocksContext>,
    /// Blocks wrappers
    #[prost(message, repeated, tag = "3")]
    pub blocks: ::prost::alloc::vec::Vec<BlockWrapper>,
}
/// Blocks context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// GetBlocksBySlotsRequest holds request for GetBlocksBySlots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksBySlotsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Slots
    #[prost(message, repeated, tag = "2")]
    pub slots: ::prost::alloc::vec::Vec<Slot>,
}
/// GetBlocksBySlotsResponse holds response from GetBlocksBySlots
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksBySlotsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Blocks
    #[prost(message, repeated, tag = "2")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
}
/// GetDatastoreEntriesRequest holds request from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<DatastoreEntriesQuery>,
}
/// DatastoreEntries Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntriesQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<DatastoreEntryFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntryFilter {
    /// / Associated address of the entry
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Datastore key
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// GetDatastoreEntriesResponse holds response from GetDatastoreEntries
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatastoreEntriesResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Datastore entries
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<DatastoreEntry>,
}
/// DatastoreEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatastoreEntry {
    /// final datastore entry value
    #[prost(bytes = "vec", tag = "1")]
    pub final_value: ::prost::alloc::vec::Vec<u8>,
    /// candidate_value datastore entry value
    #[prost(bytes = "vec", tag = "2")]
    pub candidate_value: ::prost::alloc::vec::Vec<u8>,
}
/// GetLargestStakersRequest holds request from GetLargestStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLargestStakersRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<LargestStakersQuery>,
}
/// LargestStakers Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersQuery {
    /// Starting offset for the list of stakers. Defaults to 1
    #[prost(fixed64, tag = "1")]
    pub offset: u64,
    /// Limits the number of stakers to return. Defaults to 50
    #[prost(fixed64, tag = "2")]
    pub limit: u64,
    /// Filter
    #[prost(message, optional, tag = "3")]
    pub filter: ::core::option::Option<LargestStakersFilter>,
}
/// LargestStakers Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersFilter {
    /// Minimum rolls (Optional)
    #[prost(fixed64, optional, tag = "1")]
    pub min_rolls: ::core::option::Option<u64>,
    /// Maximum rolls (Optional)
    #[prost(fixed64, optional, tag = "2")]
    pub max_rolls: ::core::option::Option<u64>,
}
/// GetLargestStakersResponse holds response from GetLargestStakers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLargestStakersResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<LargestStakersContext>,
    /// Largest stakers
    #[prost(message, repeated, tag = "3")]
    pub stakers: ::prost::alloc::vec::Vec<LargestStakerEntry>,
}
/// LargestStakers context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakersContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// LargestStakerEntry
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LargestStakerEntry {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Rolls
    #[prost(fixed64, tag = "2")]
    pub rolls: u64,
}
/// GetNextBlockBestParentsRequest holds request for GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetNextBlockBestParentsResponse holds response from GetNextBlockBestParents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNextBlockBestParentsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Best parents
    #[prost(message, repeated, tag = "2")]
    pub parents: ::prost::alloc::vec::Vec<BlockParent>,
}
/// Block parent tuple
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockParent {
    /// Block id
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
    /// Period
    #[prost(fixed64, tag = "2")]
    pub period: u64,
}
/// GetOperationsRequest holds request for GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<GetOperationsQuery>,
}
/// GetOperations Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetOperationsFilter>,
}
/// GetOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsFilter {
    /// Operation id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetOperationsResponse holds response from GetOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<OperationsContext>,
    /// Operations wrappers
    #[prost(message, repeated, tag = "3")]
    pub operations: ::prost::alloc::vec::Vec<OperationWrapper>,
}
/// Operations context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// GetScExecutionEventsRequest holds request for GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<GetScExecutionEventsQuery>,
}
/// GetScExecutionEvents Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<GetScExecutionEventsFilter>,
}
/// GetScExecutionEvents Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsFilter {
    /// Start slot
    #[prost(message, optional, tag = "1")]
    pub start_slot: ::core::option::Option<Slot>,
    /// End slot
    #[prost(message, optional, tag = "2")]
    pub end_slot: ::core::option::Option<Slot>,
    /// Caller address
    #[prost(string, optional, tag = "3")]
    pub caller_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Emitter address
    #[prost(string, optional, tag = "4")]
    pub emitter_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Original operation id
    #[prost(string, optional, tag = "5")]
    pub original_operation_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Status
    #[prost(enumeration = "ScExecutionEventStatus", repeated, tag = "6")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// GetScExecutionEventsResponse holds response from GetScExecutionEvents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Context
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<GetScExecutionEventsContext>,
    /// ScExecutionEvents
    #[prost(message, repeated, tag = "3")]
    pub events: ::prost::alloc::vec::Vec<ScExecutionEvent>,
}
/// ScExecutionEvents context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScExecutionEventsContext {
    /// Slot
    #[prost(message, optional, tag = "1")]
    pub slot: ::core::option::Option<Slot>,
}
/// GetSelectorDrawsRequest holds request from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Queries
    #[prost(message, repeated, tag = "2")]
    pub queries: ::prost::alloc::vec::Vec<SelectorDrawsQuery>,
}
/// SelectorDraws Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDrawsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<SelectorDrawsFilter>,
}
/// SelectorDraws Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDrawsFilter {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// GetSelectorDrawsResponse holds response from GetSelectorDraws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSelectorDrawsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Selector draws
    #[prost(message, repeated, tag = "2")]
    pub selector_draws: ::prost::alloc::vec::Vec<SelectorDraws>,
}
/// Selector draws
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelectorDraws {
    /// Address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Next block draws
    #[prost(message, repeated, tag = "2")]
    pub next_block_draws: ::prost::alloc::vec::Vec<Slot>,
    /// Next endorsements draws
    #[prost(message, repeated, tag = "3")]
    pub next_endorsement_draws: ::prost::alloc::vec::Vec<IndexedSlot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMipStatusRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetMipStatusResponse holds response from GetMipStatus
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMipStatusResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// MipInfo - status id entry
    #[prost(message, repeated, tag = "2")]
    pub entry: ::prost::alloc::vec::Vec<MipStatusEntry>,
}
/// GetTransactionsThroughputRequest holds request for GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetTransactionsThroughputResponse holds response from GetTransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionsThroughputResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Transactions throughput
    #[prost(fixed32, tag = "2")]
    pub throughput: u32,
}
/// GetVersionRequest holds request from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// GetVersionResponse holds response from GetVersion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// NewBlocksRequest holds request for NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewBlocksResponse holds response from NewBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed block
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<SignedBlock>,
}
/// NewBlocksHeadersRequest holds request for NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewBlocksHeadersResponse holds response from NewBlocksHeaders
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewBlocksHeadersResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed block header
    #[prost(message, optional, tag = "2")]
    pub block_header: ::core::option::Option<SignedBlockHeader>,
}
/// NewEndorsementsRequest holds request for NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewEndorsementsResponse holds response from NewEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEndorsementsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed endorsement
    #[prost(message, optional, tag = "2")]
    pub endorsement: ::core::option::Option<SignedEndorsement>,
}
/// NewFilledBlocksRequest holds request for NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// NewFilledBlocksResponse holds response from NewFilledBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewFilledBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Block with operations content
    #[prost(message, optional, tag = "2")]
    pub filled_block: ::core::option::Option<FilledBlock>,
}
/// NewOperationsRequest holds request for NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<NewOperationsQuery>,
}
/// NewOperations Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<NewOperationsFilter>,
}
/// NewOperations Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsFilter {
    /// Operation type enum
    #[prost(enumeration = "OpType", repeated, tag = "1")]
    pub types: ::prost::alloc::vec::Vec<i32>,
}
/// NewOperationsResponse holds response from NewOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Signed operation
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<SignedOperation>,
}
/// NewSlotExecutionOutputsRequest holds request for NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Query
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<NewSlotExecutionOutputsQuery>,
}
/// NewSlotExecutionOutputs Query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsQuery {
    /// Filter
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<NewSlotExecutionOutputsFilter>,
}
/// NewSlotExecutionOutputs Filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsFilter {
    /// Execution output status enum
    #[prost(enumeration = "ExecutionOutputStatus", repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
/// NewSlotExecutionOutputsResponse holds response from NewSlotExecutionOutputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSlotExecutionOutputsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Slot execution output
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<SlotExecutionOutput>,
}
/// SendBlocksRequest holds parameters to SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secure shared block
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<SecureShare>,
}
/// SendBlocksResponse holds response from SendBlocks
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBlocksResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Block result or a gRPC status
    #[prost(oneof = "send_blocks_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_blocks_response::Message>,
}
/// Nested message and enum types in `SendBlocksResponse`.
pub mod send_blocks_response {
    /// Block result or a gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Block result
        #[prost(message, tag = "2")]
        Result(super::BlockResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Block response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResult {
    /// Block id
    #[prost(string, tag = "1")]
    pub block_id: ::prost::alloc::string::String,
}
/// SendEndorsementsRequest holds parameters to SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secure shared endorsements
    #[prost(message, repeated, tag = "2")]
    pub endorsements: ::prost::alloc::vec::Vec<SecureShare>,
}
/// SendEndorsementsResponse holds response from SendEndorsements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendEndorsementsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Endorsement result or gRPC status
    #[prost(oneof = "send_endorsements_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_endorsements_response::Message>,
}
/// Nested message and enum types in `SendEndorsementsResponse`.
pub mod send_endorsements_response {
    /// Endorsement result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Endorsement result
        #[prost(message, tag = "2")]
        Result(super::EndorsementResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Endorsement response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndorsementResult {
    /// Endorsements ids
    #[prost(string, repeated, tag = "1")]
    pub endorsements_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SendOperationsRequest holds parameters to SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Secured shared operations
    #[prost(message, repeated, tag = "2")]
    pub operations: ::prost::alloc::vec::Vec<SecureShare>,
}
/// SendOperationsResponse holds response from SendOperations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendOperationsResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Operation result or gRPC status
    #[prost(oneof = "send_operations_response::Message", tags = "2, 3")]
    pub message: ::core::option::Option<send_operations_response::Message>,
}
/// Nested message and enum types in `SendOperationsResponse`.
pub mod send_operations_response {
    /// Operation result or gRPC status
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Operation result
        #[prost(message, tag = "2")]
        Result(super::OperationResult),
        /// gRPC error(status)
        #[prost(message, tag = "3")]
        Error(super::super::super::super::google::rpc::Status),
    }
}
/// Holds Operation response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationResult {
    /// Operations ids
    #[prost(string, repeated, tag = "1")]
    pub operations_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TransactionsThroughputRequest holds request for TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputRequest {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Timer interval in seconds (Optional). Defaults to 10s
    #[prost(fixed64, optional, tag = "2")]
    pub interval: ::core::option::Option<u64>,
}
/// TransactionsThroughputResponse holds response from TransactionsThroughput
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsThroughputResponse {
    /// Request id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Transactions throughput
    #[prost(fixed32, tag = "2")]
    pub throughput: u32,
}
/// Operation type enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpType {
    /// Default enum value
    Unspecified = 0,
    /// Transaction
    Transaction = 1,
    /// Roll buy
    RollBuy = 2,
    /// Roll sell
    RollSell = 3,
    /// Execute smart contract
    ExecuteSc = 4,
    /// Call smart contract
    CallSc = 5,
}
impl OpType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpType::Unspecified => "OP_TYPE_UNSPECIFIED",
            OpType::Transaction => "OP_TYPE_TRANSACTION",
            OpType::RollBuy => "OP_TYPE_ROLL_BUY",
            OpType::RollSell => "OP_TYPE_ROLL_SELL",
            OpType::ExecuteSc => "OP_TYPE_EXECUTE_SC",
            OpType::CallSc => "OP_TYPE_CALL_SC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OP_TYPE_TRANSACTION" => Some(Self::Transaction),
            "OP_TYPE_ROLL_BUY" => Some(Self::RollBuy),
            "OP_TYPE_ROLL_SELL" => Some(Self::RollSell),
            "OP_TYPE_EXECUTE_SC" => Some(Self::ExecuteSc),
            "OP_TYPE_CALL_SC" => Some(Self::CallSc),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod massa_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Massa gRPC service
    #[derive(Debug, Clone)]
    pub struct MassaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MassaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MassaServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MassaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MassaServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Get blocks by ids
        pub async fn get_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetBlocks"));
            self.inner.unary(req, path, codec).await
        }
        /// Get blocks by slots
        pub async fn get_blocks_by_slots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlocksBySlotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksBySlotsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetBlocksBySlots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetBlocksBySlots"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get datastore entries
        pub async fn get_datastore_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetDatastoreEntries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetDatastoreEntries"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get largest stakers
        pub async fn get_largest_stakers(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLargestStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLargestStakersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetLargestStakers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetLargestStakers"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get next block best parents
        pub async fn get_next_block_best_parents(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetNextBlockBestParents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "GetNextBlockBestParents",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get operations
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetOperations"));
            self.inner.unary(req, path, codec).await
        }
        /// Get smart contracts execution events
        pub async fn get_sc_execution_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetScExecutionEvents",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetScExecutionEvents"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get selector draws
        pub async fn get_selector_draws(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetSelectorDraws",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "GetSelectorDraws"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get transactions throughput
        pub async fn get_transactions_throughput(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetTransactionsThroughput",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "GetTransactionsThroughput",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get node version
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// Get
        pub async fn get_mip_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMipStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMipStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/GetMipStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "GetMipStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// New received and produced blocks
        pub async fn new_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::NewBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced blocks headers
        pub async fn new_blocks_headers(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewBlocksHeadersRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewBlocksHeadersResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewBlocksHeaders",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "NewBlocksHeaders"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced endorsements
        pub async fn new_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewEndorsementsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewEndorsements"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced blocks with operations
        pub async fn new_filled_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewFilledBlocksRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewFilledBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewFilledBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewFilledBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and produced operations
        pub async fn new_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::NewOperationsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "NewOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// New received and slot execution events
        pub async fn new_slot_execution_outputs(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::NewSlotExecutionOutputsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::NewSlotExecutionOutputsResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/NewSlotExecutionOutputs",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "NewSlotExecutionOutputs",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send blocks
        pub async fn send_blocks(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendBlocksResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendBlocks",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "SendBlocks"));
            self.inner.streaming(req, path, codec).await
        }
        /// Send endorsements
        pub async fn send_endorsements(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendEndorsementsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendEndorsementsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendEndorsements",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("massa.api.v1.MassaService", "SendEndorsements"),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Send operations
        pub async fn send_operations(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::SendOperationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendOperationsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/SendOperations",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("massa.api.v1.MassaService", "SendOperations"));
            self.inner.streaming(req, path, codec).await
        }
        /// Transactions throughput
        pub async fn transactions_throughput(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::TransactionsThroughputRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::TransactionsThroughputResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/massa.api.v1.MassaService/TransactionsThroughput",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "massa.api.v1.MassaService",
                        "TransactionsThroughput",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod massa_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MassaServiceServer.
    #[async_trait]
    pub trait MassaService: Send + Sync + 'static {
        /// Get blocks by ids
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksResponse>,
            tonic::Status,
        >;
        /// Get blocks by slots
        async fn get_blocks_by_slots(
            &self,
            request: tonic::Request<super::GetBlocksBySlotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBlocksBySlotsResponse>,
            tonic::Status,
        >;
        /// Get datastore entries
        async fn get_datastore_entries(
            &self,
            request: tonic::Request<super::GetDatastoreEntriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatastoreEntriesResponse>,
            tonic::Status,
        >;
        /// Get largest stakers
        async fn get_largest_stakers(
            &self,
            request: tonic::Request<super::GetLargestStakersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLargestStakersResponse>,
            tonic::Status,
        >;
        /// Get next block best parents
        async fn get_next_block_best_parents(
            &self,
            request: tonic::Request<super::GetNextBlockBestParentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNextBlockBestParentsResponse>,
            tonic::Status,
        >;
        /// Get operations
        async fn get_operations(
            &self,
            request: tonic::Request<super::GetOperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsResponse>,
            tonic::Status,
        >;
        /// Get smart contracts execution events
        async fn get_sc_execution_events(
            &self,
            request: tonic::Request<super::GetScExecutionEventsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetScExecutionEventsResponse>,
            tonic::Status,
        >;
        /// Get selector draws
        async fn get_selector_draws(
            &self,
            request: tonic::Request<super::GetSelectorDrawsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSelectorDrawsResponse>,
            tonic::Status,
        >;
        /// Get transactions throughput
        async fn get_transactions_throughput(
            &self,
            request: tonic::Request<super::GetTransactionsThroughputRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTransactionsThroughputResponse>,
            tonic::Status,
        >;
        /// Get node version
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVersionResponse>,
            tonic::Status,
        >;
        /// Get
        async fn get_mip_status(
            &self,
            request: tonic::Request<super::GetMipStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMipStatusResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewBlocks method.
        type NewBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks
        async fn new_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::NewBlocksStream>, tonic::Status>;
        /// Server streaming response type for the NewBlocksHeaders method.
        type NewBlocksHeadersStream: futures_core::Stream<
                Item = std::result::Result<
                    super::NewBlocksHeadersResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// New received and produced blocks headers
        async fn new_blocks_headers(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewBlocksHeadersRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewBlocksHeadersStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewEndorsements method.
        type NewEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<super::NewEndorsementsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced endorsements
        async fn new_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewFilledBlocks method.
        type NewFilledBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::NewFilledBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced blocks with operations
        async fn new_filled_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewFilledBlocksRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewFilledBlocksStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewOperations method.
        type NewOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::NewOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// New received and produced operations
        async fn new_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::NewOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::NewOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewSlotExecutionOutputs method.
        type NewSlotExecutionOutputsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::NewSlotExecutionOutputsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// New received and slot execution events
        async fn new_slot_execution_outputs(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::NewSlotExecutionOutputsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendBlocks method.
        type SendBlocksStream: futures_core::Stream<
                Item = std::result::Result<super::SendBlocksResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send blocks
        async fn send_blocks(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendBlocksRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SendBlocksStream>, tonic::Status>;
        /// Server streaming response type for the SendEndorsements method.
        type SendEndorsementsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::SendEndorsementsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Send endorsements
        async fn send_endorsements(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendEndorsementsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendEndorsementsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the SendOperations method.
        type SendOperationsStream: futures_core::Stream<
                Item = std::result::Result<super::SendOperationsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send operations
        async fn send_operations(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendOperationsRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::SendOperationsStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the TransactionsThroughput method.
        type TransactionsThroughputStream: futures_core::Stream<
                Item = std::result::Result<
                    super::TransactionsThroughputResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Transactions throughput
        async fn transactions_throughput(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::TransactionsThroughputRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::TransactionsThroughputStream>,
            tonic::Status,
        >;
    }
    /// Massa gRPC service
    #[derive(Debug)]
    pub struct MassaServiceServer<T: MassaService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MassaService> MassaServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MassaServiceServer<T>
    where
        T: MassaService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/massa.api.v1.MassaService/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetBlocksRequest>
                    for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetBlocksBySlots" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksBySlotsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetBlocksBySlotsRequest>
                    for GetBlocksBySlotsSvc<T> {
                        type Response = super::GetBlocksBySlotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksBySlotsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_blocks_by_slots(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksBySlotsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetDatastoreEntries" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatastoreEntriesSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetDatastoreEntriesRequest>
                    for GetDatastoreEntriesSvc<T> {
                        type Response = super::GetDatastoreEntriesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatastoreEntriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_datastore_entries(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatastoreEntriesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetLargestStakers" => {
                    #[allow(non_camel_case_types)]
                    struct GetLargestStakersSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetLargestStakersRequest>
                    for GetLargestStakersSvc<T> {
                        type Response = super::GetLargestStakersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLargestStakersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_largest_stakers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLargestStakersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetNextBlockBestParents" => {
                    #[allow(non_camel_case_types)]
                    struct GetNextBlockBestParentsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetNextBlockBestParentsRequest>
                    for GetNextBlockBestParentsSvc<T> {
                        type Response = super::GetNextBlockBestParentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetNextBlockBestParentsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_next_block_best_parents(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNextBlockBestParentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetOperations" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetOperationsRequest>
                    for GetOperationsSvc<T> {
                        type Response = super::GetOperationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOperationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetScExecutionEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetScExecutionEventsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetScExecutionEventsRequest>
                    for GetScExecutionEventsSvc<T> {
                        type Response = super::GetScExecutionEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetScExecutionEventsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_sc_execution_events(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetScExecutionEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetSelectorDraws" => {
                    #[allow(non_camel_case_types)]
                    struct GetSelectorDrawsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetSelectorDrawsRequest>
                    for GetSelectorDrawsSvc<T> {
                        type Response = super::GetSelectorDrawsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSelectorDrawsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_selector_draws(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSelectorDrawsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetTransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionsThroughputSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<
                        super::GetTransactionsThroughputRequest,
                    > for GetTransactionsThroughputSvc<T> {
                        type Response = super::GetTransactionsThroughputResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetTransactionsThroughputRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_transactions_throughput(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionsThroughputSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetVersionRequest>
                    for GetVersionSvc<T> {
                        type Response = super::GetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/GetMipStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetMipStatusSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::UnaryService<super::GetMipStatusRequest>
                    for GetMipStatusSvc<T> {
                        type Response = super::GetMipStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMipStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_mip_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMipStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewBlocksRequest>
                    for NewBlocksSvc<T> {
                        type Response = super::NewBlocksResponse;
                        type ResponseStream = T::NewBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewBlocksHeaders" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksHeadersSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewBlocksHeadersRequest>
                    for NewBlocksHeadersSvc<T> {
                        type Response = super::NewBlocksHeadersResponse;
                        type ResponseStream = T::NewBlocksHeadersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewBlocksHeadersRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_blocks_headers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewBlocksHeadersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct NewEndorsementsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewEndorsementsRequest>
                    for NewEndorsementsSvc<T> {
                        type Response = super::NewEndorsementsResponse;
                        type ResponseStream = T::NewEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_endorsements(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEndorsementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewFilledBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewFilledBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewFilledBlocksRequest>
                    for NewFilledBlocksSvc<T> {
                        type Response = super::NewFilledBlocksResponse;
                        type ResponseStream = T::NewFilledBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewFilledBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_filled_blocks(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewFilledBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewOperations" => {
                    #[allow(non_camel_case_types)]
                    struct NewOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::NewOperationsRequest>
                    for NewOperationsSvc<T> {
                        type Response = super::NewOperationsResponse;
                        type ResponseStream = T::NewOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/NewSlotExecutionOutputs" => {
                    #[allow(non_camel_case_types)]
                    struct NewSlotExecutionOutputsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<
                        super::NewSlotExecutionOutputsRequest,
                    > for NewSlotExecutionOutputsSvc<T> {
                        type Response = super::NewSlotExecutionOutputsResponse;
                        type ResponseStream = T::NewSlotExecutionOutputsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NewSlotExecutionOutputsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_slot_execution_outputs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewSlotExecutionOutputsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct SendBlocksSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendBlocksRequest>
                    for SendBlocksSvc<T> {
                        type Response = super::SendBlocksResponse;
                        type ResponseStream = T::SendBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendBlocksRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).send_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendEndorsements" => {
                    #[allow(non_camel_case_types)]
                    struct SendEndorsementsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendEndorsementsRequest>
                    for SendEndorsementsSvc<T> {
                        type Response = super::SendEndorsementsResponse;
                        type ResponseStream = T::SendEndorsementsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendEndorsementsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_endorsements(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendEndorsementsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/SendOperations" => {
                    #[allow(non_camel_case_types)]
                    struct SendOperationsSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<super::SendOperationsRequest>
                    for SendOperationsSvc<T> {
                        type Response = super::SendOperationsResponse;
                        type ResponseStream = T::SendOperationsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::SendOperationsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).send_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendOperationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/massa.api.v1.MassaService/TransactionsThroughput" => {
                    #[allow(non_camel_case_types)]
                    struct TransactionsThroughputSvc<T: MassaService>(pub Arc<T>);
                    impl<
                        T: MassaService,
                    > tonic::server::StreamingService<
                        super::TransactionsThroughputRequest,
                    > for TransactionsThroughputSvc<T> {
                        type Response = super::TransactionsThroughputResponse;
                        type ResponseStream = T::TransactionsThroughputStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::TransactionsThroughputRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).transactions_throughput(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransactionsThroughputSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MassaService> Clone for MassaServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MassaService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MassaService> tonic::server::NamedService for MassaServiceServer<T> {
        const NAME: &'static str = "massa.api.v1.MassaService";
    }
}
