/// A transactional get command. Lookup a value for `key` in the transaction with
/// starting timestamp = `version`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// A region error indicates that the request was sent to the wrong TiKV node
    /// (or other, similar errors).
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<Error>,
    /// A value could not be retrieved due to the state of the database for the requested key.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// A successful result.
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// True if the key does not exist in the database.
    #[prost(bool, tag = "4")]
    pub not_found: bool,
    /// Time and scan details when processing the request.
    #[prost(message, optional, tag = "6")]
    pub exec_details_v2: ::core::option::Option<ExecDetailsV2>,
}
// Helper messages.

/// Miscellaneous metadata attached to most requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub region_epoch: ::core::option::Option<RegionEpoch>,
    #[prost(message, optional, tag = "3")]
    pub peer: ::core::option::Option<Peer>,
    #[prost(uint64, tag = "5")]
    pub term: u64,
    #[prost(enumeration = "CommandPri", tag = "6")]
    pub priority: i32,
    #[prost(enumeration = "IsolationLevel", tag = "7")]
    pub isolation_level: i32,
    #[prost(bool, tag = "8")]
    pub not_fill_cache: bool,
    #[prost(bool, tag = "9")]
    pub sync_log: bool,
    /// True means execution time statistics should be recorded and returned.
    #[prost(bool, tag = "10")]
    pub record_time_stat: bool,
    /// True means RocksDB scan statistics should be recorded and returned.
    #[prost(bool, tag = "11")]
    pub record_scan_stat: bool,
    #[prost(bool, tag = "12")]
    pub replica_read: bool,
    #[prost(uint64, repeated, tag = "13")]
    pub resolved_locks: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "14")]
    pub max_execution_duration_ms: u64,
    /// After a region applies to `applied_index`, we can get a
    /// snapshot for the region even if the peer is a follower.
    #[prost(uint64, tag = "15")]
    pub applied_index: u64,
    /// A hint for TiKV to schedule tasks more fairly. Query with same task ID
    /// may share same priority and resource quota.
    #[prost(uint64, tag = "16")]
    pub task_id: u64,
    /// Not required to read the most up-to-date data, replicas with `safe_ts` >= `start_ts`
    /// can handle read request directly
    #[prost(bool, tag = "17")]
    pub stale_read: bool,
    /// Any additional serialized information about the request.
    #[prost(bytes = "vec", tag = "18")]
    pub resource_group_tag: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub lock_version: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub lock_ttl: u64,
    /// How many keys this transaction involves in this region.
    #[prost(uint64, tag = "5")]
    pub txn_size: u64,
    #[prost(enumeration = "Op", tag = "6")]
    pub lock_type: i32,
    #[prost(uint64, tag = "7")]
    pub lock_for_update_ts: u64,
    /// Fields for transactions that are using Async Commit.
    #[prost(bool, tag = "8")]
    pub use_async_commit: bool,
    #[prost(uint64, tag = "9")]
    pub min_commit_ts: u64,
    #[prost(bytes = "vec", repeated, tag = "10")]
    pub secondaries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyError {
    /// Client should backoff or cleanup the lock then retry.
    #[prost(message, optional, tag = "1")]
    pub locked: ::core::option::Option<LockInfo>,
    /// Client may restart the txn. e.g write conflict.
    #[prost(string, tag = "2")]
    pub retryable: ::prost::alloc::string::String,
    /// Client should abort the txn.
    #[prost(string, tag = "3")]
    pub abort: ::prost::alloc::string::String,
    /// Write conflict is moved from retryable to here.
    #[prost(message, optional, tag = "4")]
    pub conflict: ::core::option::Option<WriteConflict>,
    /// Key already exists
    #[prost(message, optional, tag = "5")]
    pub already_exist: ::core::option::Option<AlreadyExist>,
    /// Deadlock is used in pessimistic transaction for single statement rollback.
    #[prost(message, optional, tag = "6")]
    pub deadlock: ::core::option::Option<Deadlock>,
    /// Commit ts is earlier than min commit ts of a transaction.
    #[prost(message, optional, tag = "7")]
    pub commit_ts_expired: ::core::option::Option<CommitTsExpired>,
    /// Txn not found when checking txn status.
    #[prost(message, optional, tag = "8")]
    pub txn_not_found: ::core::option::Option<TxnNotFound>,
    /// Calculated commit TS exceeds the limit given by the user.
    #[prost(message, optional, tag = "9")]
    pub commit_ts_too_large: ::core::option::Option<CommitTsTooLarge>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteConflict {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub conflict_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub primary: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub conflict_commit_ts: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlreadyExist {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deadlock {
    #[prost(uint64, tag = "1")]
    pub lock_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub lock_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub deadlock_key_hash: u64,
    #[prost(message, repeated, tag = "4")]
    pub wait_chain: ::prost::alloc::vec::Vec<WaitForEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForEntry {
    /// The transaction id that is waiting.
    #[prost(uint64, tag = "1")]
    pub txn: u64,
    /// The transaction id that is being waited for.
    #[prost(uint64, tag = "2")]
    pub wait_for_txn: u64,
    /// The hash value of the key is being waited for.
    #[prost(uint64, tag = "3")]
    pub key_hash: u64,
    /// The key the current txn is trying to lock.
    #[prost(bytes = "vec", tag = "4")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// The tag came from the lock request's context.
    #[prost(bytes = "vec", tag = "5")]
    pub resource_group_tag: ::prost::alloc::vec::Vec<u8>,
    /// Milliseconds it has been waits.
    #[prost(uint64, tag = "6")]
    pub wait_time: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTsExpired {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub attempted_commit_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub min_commit_ts: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnNotFound {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub primary_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTsTooLarge {
    /// The calculated commit TS.
    #[prost(uint64, tag = "1")]
    pub commit_ts: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeDetail {
    /// Off-cpu wall time elapsed in TiKV side. Usually this includes queue waiting time and
    /// other kind of waitings in series.
    #[prost(int64, tag = "1")]
    pub wait_wall_time_ms: i64,
    /// Off-cpu and on-cpu wall time elapsed to actually process the request payload. It does not
    /// include `wait_wall_time`.
    /// This field is very close to the CPU time in most cases. Some wait time spend in RocksDB
    /// cannot be excluded for now, like Mutex wait time, which is included in this field, so that
    /// this field is called wall time instead of CPU time.
    #[prost(int64, tag = "2")]
    pub process_wall_time_ms: i64,
    /// KV read wall Time means the time used in key/value scan and get.
    #[prost(int64, tag = "3")]
    pub kv_read_wall_time_ms: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanDetailV2 {
    /// Number of user keys scanned from the storage.
    /// It does not include deleted version or RocksDB tombstone keys.
    /// For Coprocessor requests, it includes keys that has been filtered out by
    /// Selection.
    #[prost(uint64, tag = "1")]
    pub processed_versions: u64,
    /// Number of bytes of user key-value pairs scanned from the storage, i.e.
    /// total size of data returned from MVCC layer.
    #[prost(uint64, tag = "8")]
    pub processed_versions_size: u64,
    /// Approximate number of MVCC keys meet during scanning. It includes
    /// deleted versions, but does not include RocksDB tombstone keys.
    ///
    /// When this field is notably larger than `processed_versions`, it means
    /// there are a lot of deleted MVCC keys.
    #[prost(uint64, tag = "2")]
    pub total_versions: u64,
    /// Total number of deletes and single deletes skipped over during
    /// iteration, i.e. how many RocksDB tombstones are skipped.
    #[prost(uint64, tag = "3")]
    pub rocksdb_delete_skipped_count: u64,
    /// Total number of internal keys skipped over during iteration.
    /// See https://github.com/facebook/rocksdb/blob/9f1c84ca471d8b1ad7be9f3eebfc2c7e07dfd7a7/include/rocksdb/perf_context.h#L84 for details.
    #[prost(uint64, tag = "4")]
    pub rocksdb_key_skipped_count: u64,
    /// Total number of RocksDB block cache hits.
    #[prost(uint64, tag = "5")]
    pub rocksdb_block_cache_hit_count: u64,
    /// Total number of block reads (with IO).
    #[prost(uint64, tag = "6")]
    pub rocksdb_block_read_count: u64,
    /// Total number of bytes from block reads.
    #[prost(uint64, tag = "7")]
    pub rocksdb_block_read_byte: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecDetailsV2 {
    /// Available when ctx.record_time_stat = true or meet slow query.
    #[prost(message, optional, tag = "1")]
    pub time_detail: ::core::option::Option<TimeDetail>,
    /// Available when ctx.record_scan_stat = true or meet slow query.
    #[prost(message, optional, tag = "2")]
    pub scan_detail_v2: ::core::option::Option<ScanDetailV2>,
}
/// Error wraps all region errors, indicates an error encountered by a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    /// The error message
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub not_leader: ::core::option::Option<NotLeader>,
    #[prost(message, optional, tag = "3")]
    pub region_not_found: ::core::option::Option<RegionNotFound>,
    #[prost(message, optional, tag = "4")]
    pub key_not_in_region: ::core::option::Option<KeyNotInRegion>,
    #[prost(message, optional, tag = "5")]
    pub epoch_not_match: ::core::option::Option<EpochNotMatch>,
    #[prost(message, optional, tag = "6")]
    pub server_is_busy: ::core::option::Option<ServerIsBusy>,
    #[prost(message, optional, tag = "7")]
    pub stale_command: ::core::option::Option<StaleCommand>,
    #[prost(message, optional, tag = "8")]
    pub store_not_match: ::core::option::Option<StoreNotMatch>,
    #[prost(message, optional, tag = "9")]
    pub raft_entry_too_large: ::core::option::Option<RaftEntryTooLarge>,
    #[prost(message, optional, tag = "10")]
    pub max_timestamp_not_synced: ::core::option::Option<MaxTimestampNotSynced>,
    #[prost(message, optional, tag = "11")]
    pub read_index_not_ready: ::core::option::Option<ReadIndexNotReady>,
    #[prost(message, optional, tag = "12")]
    pub proposal_in_merging_mode: ::core::option::Option<ProposalInMergingMode>,
    #[prost(message, optional, tag = "13")]
    pub data_is_not_ready: ::core::option::Option<DataIsNotReady>,
    #[prost(message, optional, tag = "14")]
    pub region_not_initialized: ::core::option::Option<RegionNotInitialized>,
}
/// NotLeader is the error variant that tells a request be handle by raft leader
/// is sent to raft follower or learner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotLeader {
    /// The requested region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    /// Region leader of the requested region
    #[prost(message, optional, tag = "2")]
    pub leader: ::core::option::Option<Peer>,
}
/// StoreNotMatch is the error variant that tells the request is sent to wrong store.
/// (i.e. inconsistency of the store ID that request shows and the real store ID of this server.)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreNotMatch {
    /// Store id in request
    #[prost(uint64, tag = "1")]
    pub request_store_id: u64,
    /// Actual store id
    #[prost(uint64, tag = "2")]
    pub actual_store_id: u64,
}
/// RegionNotFound is the error variant that tells there isn't any region in this TiKV
/// matches the requested region ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionNotFound {
    /// The requested region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
/// RegionNotInitialized is the error variant that tells there isn't any initialized peer
/// matchesthe request region ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionNotInitialized {
    /// The request region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
/// KeyNotInRegion is the error variant that tells the key the request requires isn't present in
/// this region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyNotInRegion {
    /// The requested key
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// The requested region ID
    #[prost(uint64, tag = "2")]
    pub region_id: u64,
    /// Start key of the requested region
    #[prost(bytes = "vec", tag = "3")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    /// Snd key of the requested region
    #[prost(bytes = "vec", tag = "4")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
/// EpochNotMatch is the error variant that tells a region has been updated.
/// (e.g. by splitting / merging, or raft Confchange.)
/// Hence, a command is based on a stale version of a region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochNotMatch {
    /// Available regions that may be siblings of the requested one.
    #[prost(message, repeated, tag = "1")]
    pub current_regions: ::prost::alloc::vec::Vec<Region>,
}
/// ServerIsBusy is the error variant that tells the server is too busy to response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerIsBusy {
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    /// The suggested backoff time
    #[prost(uint64, tag = "2")]
    pub backoff_ms: u64,
}
/// StaleCommand is the error variant that tells the command is stale, that is,
/// the current request term is lower than current raft term.
/// This can be retried at most time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaleCommand {}
/// RaftEntryTooLarge is the error variant that tells the request is too large to be serialized to a
/// reasonable small raft entry.
/// (i.e. greater than the configured value `raft_entry_max_size` in `raftstore`)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftEntryTooLarge {
    /// The requested region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    /// Size of the raft entry
    #[prost(uint64, tag = "2")]
    pub entry_size: u64,
}
/// MaxTimestampNotSynced is the error variant that tells the peer has just become a leader and
/// updating the max timestamp in the concurrency manager from PD TSO is ongoing. In this case,
/// the prewrite of an async commit transaction cannot succeed. The client can backoff and
/// resend the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxTimestampNotSynced {}
/// ReadIndexNotReady is the error variant that tells the read index request is not ready, that is,
/// the current region is in a status that not ready to serve the read index request. For example,
/// region is in splitting or merging status.
/// This can be retried at most time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadIndexNotReady {
    /// The reason why the region is not ready to serve read index request
    #[prost(string, tag = "1")]
    pub reason: ::prost::alloc::string::String,
    /// The requested region ID
    #[prost(uint64, tag = "2")]
    pub region_id: u64,
}
/// ProposalInMergingMode is the error variant that tells the proposal is rejected because raft is
/// in the merging mode. This may happen when BR/Lightning try to ingest SST.
/// This can be retried at most time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalInMergingMode {
    /// The requested region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataIsNotReady {
    /// The requested region ID
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub peer_id: u64,
    #[prost(uint64, tag = "3")]
    pub safe_ts: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Region key range [start_key, end_key).
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub region_epoch: ::core::option::Option<RegionEpoch>,
    #[prost(message, repeated, tag = "5")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
    /// Encryption metadata for start_key and end_key. encryption_meta.iv is IV for start_key.
    /// IV for end_key is calculated from (encryption_meta.iv + len(start_key)).
    /// The field is only used by PD and should be ignored otherwise.
    /// If encryption_meta is empty (i.e. nil), it means start_key and end_key are unencrypted.
    #[prost(message, optional, tag = "6")]
    pub encryption_meta: ::core::option::Option<EncryptionMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionEpoch {
    /// Conf change version, auto increment when add or remove peer
    #[prost(uint64, tag = "1")]
    pub conf_ver: u64,
    /// Region version, auto increment when split or merge
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
    #[prost(enumeration = "PeerRole", tag = "3")]
    pub role: i32,
}
/// General encryption metadata for any data type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionMeta {
    /// ID of the key used to encrypt the data.
    #[prost(uint64, tag = "1")]
    pub key_id: u64,
    /// Initialization vector (IV) of the data.
    #[prost(bytes = "vec", tag = "2")]
    pub iv: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandPri {
    /// Normal is the default value.
    Normal = 0,
    Low = 1,
    High = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IsolationLevel {
    /// SI = snapshot isolation
    Si = 0,
    /// RC = read committed
    Rc = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
    Rollback = 3,
    /// insert operation has a constraint that key should not exist before.
    Insert = 4,
    PessimisticLock = 5,
    CheckNotExists = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerRole {
    /// Voter -> Voter
    Voter = 0,
    /// Learner/None -> Learner
    Learner = 1,
    /// Learner/None -> Voter
    IncomingVoter = 2,
    /// Voter -> Learner
    ///
    /// We forbid Voter -> None, it can introduce unavailability as discussed in
    /// etcd-io/etcd#7625
    /// Learner -> None can be apply directly, doesn't need to be stored as
    /// joint state.
    DemotingVoter = 3,
}
#[doc = r" Generated client implementations."]
pub mod tikv_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Key/value store API for TiKV."]
    #[derive(Debug, Clone)]
    pub struct TikvClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TikvClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TikvClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> TikvClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TikvClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Commands using a transactional interface."]
        pub async fn kv_get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/tikvpb.Tikv/KvGet");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod tikv_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TikvServer."]
    #[async_trait]
    pub trait Tikv: Send + Sync + 'static {
        #[doc = " Commands using a transactional interface."]
        async fn kv_get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
    }
    #[doc = " Key/value store API for TiKV."]
    #[derive(Debug)]
    pub struct TikvServer<T: Tikv> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Tikv> TikvServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TikvServer<T>
    where
        T: Tikv,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/tikvpb.Tikv/KvGet" => {
                    #[allow(non_camel_case_types)]
                    struct KvGetSvc<T: Tikv>(pub Arc<T>);
                    impl<T: Tikv> tonic::server::UnaryService<super::GetRequest> for KvGetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).kv_get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = KvGetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Tikv> Clone for TikvServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Tikv> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Tikv> tonic::transport::NamedService for TikvServer<T> {
        const NAME: &'static str = "tikvpb.Tikv";
    }
}
