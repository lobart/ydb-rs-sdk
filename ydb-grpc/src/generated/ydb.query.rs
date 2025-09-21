#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
    /// Identifier of created session
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    /// Identifier node where session was created
    #[prost(int64, tag = "4")]
    pub node_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionRequest {
    /// Identifier of session to delete (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachSessionRequest {
    /// Identifier of session to attach (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionState {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerializableModeSettings {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnlineModeSettings {
    #[prost(bool, tag = "1")]
    pub allow_inconsistent_reads: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaleModeSettings {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotModeSettings {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSettings {
    #[prost(oneof = "transaction_settings::TxMode", tags = "1, 2, 3, 4")]
    pub tx_mode: ::core::option::Option<transaction_settings::TxMode>,
}
/// Nested message and enum types in `TransactionSettings`.
pub mod transaction_settings {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TxMode {
        #[prost(message, tag = "1")]
        SerializableReadWrite(super::SerializableModeSettings),
        #[prost(message, tag = "2")]
        OnlineReadOnly(super::OnlineModeSettings),
        #[prost(message, tag = "3")]
        StaleReadOnly(super::StaleModeSettings),
        #[prost(message, tag = "4")]
        SnapshotReadOnly(super::SnapshotModeSettings),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionControl {
    #[prost(bool, tag = "10")]
    pub commit_tx: bool,
    #[prost(oneof = "transaction_control::TxSelector", tags = "1, 2")]
    pub tx_selector: ::core::option::Option<transaction_control::TxSelector>,
}
/// Nested message and enum types in `TransactionControl`.
pub mod transaction_control {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TxSelector {
        #[prost(string, tag = "1")]
        TxId(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        BeginTx(super::TransactionSettings),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginTransactionRequest {
    /// Session identifier (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub tx_settings: ::core::option::Option<TransactionSettings>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionMeta {
    /// Transaction identifier
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginTransactionResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
    #[prost(message, optional, tag = "3")]
    pub tx_meta: ::core::option::Option<TransactionMeta>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTransactionRequest {
    /// Session identifier (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    /// Transaction identifier (required)
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTransactionResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackTransactionRequest {
    /// Session identifier (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    /// Transaction identifier (required)
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackTransactionResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContent {
    #[prost(enumeration = "Syntax", tag = "1")]
    pub syntax: i32,
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryRequest {
    /// Session identifier (required)
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ExecMode", tag = "2")]
    pub exec_mode: i32,
    #[prost(message, optional, tag = "3")]
    pub tx_control: ::core::option::Option<TransactionControl>,
    #[prost(map = "string, message", tag = "6")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::TypedValue,
    >,
    #[prost(enumeration = "StatsMode", tag = "7")]
    pub stats_mode: i32,
    /// For queries with multiple result sets, some of them may be computed concurrently.
    /// If true, parts of different results sets may be interleaved in response stream.
    #[prost(bool, tag = "8")]
    pub concurrent_result_sets: bool,
    #[prost(oneof = "execute_query_request::Query", tags = "4")]
    pub query: ::core::option::Option<execute_query_request::Query>,
}
/// Nested message and enum types in `ExecuteQueryRequest`.
pub mod execute_query_request {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag = "4")]
        QueryContent(super::QueryContent),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultSetMeta {
    #[prost(message, repeated, tag = "1")]
    pub columns: ::prost::alloc::vec::Vec<super::Column>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryResponsePart {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
    /// Index of current result set
    #[prost(int64, tag = "3")]
    pub result_set_index: i64,
    /// Result set part
    #[prost(message, optional, tag = "4")]
    pub result_set: ::core::option::Option<super::ResultSet>,
    /// Execution statistics (last part only)
    #[prost(message, optional, tag = "5")]
    pub exec_stats: ::core::option::Option<super::table_stats::QueryStats>,
    #[prost(message, optional, tag = "6")]
    pub tx_meta: ::core::option::Option<TransactionMeta>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptRequest {
    #[prost(message, optional, tag = "1")]
    pub operation_params: ::core::option::Option<super::operations::OperationParams>,
    #[prost(enumeration = "ExecMode", tag = "2")]
    pub exec_mode: i32,
    #[prost(message, optional, tag = "3")]
    pub script_content: ::core::option::Option<QueryContent>,
    #[prost(map = "string, message", tag = "4")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::TypedValue,
    >,
    #[prost(enumeration = "StatsMode", tag = "5")]
    pub stats_mode: i32,
    /// After script execution operation finishes, TTL will start counting.
    /// After this TTL the results will be removed from database.
    #[prost(message, optional, tag = "6")]
    pub results_ttl: ::core::option::Option<super::super::google::protobuf::Duration>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteScriptMetadata {
    #[prost(string, tag = "1")]
    pub execution_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ExecStatus", tag = "2")]
    pub exec_status: i32,
    #[prost(message, optional, tag = "3")]
    pub script_content: ::core::option::Option<QueryContent>,
    #[prost(message, repeated, tag = "4")]
    pub result_sets_meta: ::prost::alloc::vec::Vec<ResultSetMeta>,
    #[prost(enumeration = "ExecMode", tag = "5")]
    pub exec_mode: i32,
    /// Execution statistics
    #[prost(message, optional, tag = "6")]
    pub exec_stats: ::core::option::Option<super::table_stats::QueryStats>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchScriptResultsRequest {
    #[prost(string, tag = "1")]
    pub operation_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub result_set_index: i64,
    #[prost(string, tag = "3")]
    pub fetch_token: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub rows_limit: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchScriptResultsResponse {
    #[prost(enumeration = "super::status_ids::StatusCode", tag = "1")]
    pub status: i32,
    #[prost(message, repeated, tag = "2")]
    pub issues: ::prost::alloc::vec::Vec<super::issue::IssueMessage>,
    #[prost(int64, tag = "3")]
    pub result_set_index: i64,
    #[prost(message, optional, tag = "4")]
    pub result_set: ::core::option::Option<super::ResultSet>,
    #[prost(string, tag = "5")]
    pub next_fetch_token: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Script {
    #[prost(message, optional, tag = "1")]
    pub script_content: ::core::option::Option<QueryContent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Syntax {
    Unspecified = 0,
    /// YQL
    YqlV1 = 1,
    /// PostgresQL
    Pg = 2,
}
impl Syntax {
    /// String value of the enum field names used in the ProtoBuf definition.
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Syntax::Unspecified => "SYNTAX_UNSPECIFIED",
            Syntax::YqlV1 => "SYNTAX_YQL_V1",
            Syntax::Pg => "SYNTAX_PG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SYNTAX_UNSPECIFIED" => Some(Self::Unspecified),
            "SYNTAX_YQL_V1" => Some(Self::YqlV1),
            "SYNTAX_PG" => Some(Self::Pg),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecMode {
    Unspecified = 0,
    Parse = 10,
    Validate = 20,
    Explain = 30,
    Execute = 50,
}
impl ExecMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecMode::Unspecified => "EXEC_MODE_UNSPECIFIED",
            ExecMode::Parse => "EXEC_MODE_PARSE",
            ExecMode::Validate => "EXEC_MODE_VALIDATE",
            ExecMode::Explain => "EXEC_MODE_EXPLAIN",
            ExecMode::Execute => "EXEC_MODE_EXECUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXEC_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXEC_MODE_PARSE" => Some(Self::Parse),
            "EXEC_MODE_VALIDATE" => Some(Self::Validate),
            "EXEC_MODE_EXPLAIN" => Some(Self::Explain),
            "EXEC_MODE_EXECUTE" => Some(Self::Execute),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatsMode {
    Unspecified = 0,
    /// Stats collection is disabled
    None = 10,
    /// Aggregated stats of reads, updates and deletes per table
    Basic = 20,
    /// Add execution stats and plan on top of STATS_MODE_BASIC
    Full = 30,
    /// Detailed execution stats including stats for individual tasks and channels
    Profile = 40,
}
impl StatsMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatsMode::Unspecified => "STATS_MODE_UNSPECIFIED",
            StatsMode::None => "STATS_MODE_NONE",
            StatsMode::Basic => "STATS_MODE_BASIC",
            StatsMode::Full => "STATS_MODE_FULL",
            StatsMode::Profile => "STATS_MODE_PROFILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATS_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "STATS_MODE_NONE" => Some(Self::None),
            "STATS_MODE_BASIC" => Some(Self::Basic),
            "STATS_MODE_FULL" => Some(Self::Full),
            "STATS_MODE_PROFILE" => Some(Self::Profile),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecStatus {
    Unspecified = 0,
    Starting = 10,
    Aborted = 20,
    Cancelled = 30,
    Completed = 40,
    Failed = 50,
}
impl ExecStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecStatus::Unspecified => "EXEC_STATUS_UNSPECIFIED",
            ExecStatus::Starting => "EXEC_STATUS_STARTING",
            ExecStatus::Aborted => "EXEC_STATUS_ABORTED",
            ExecStatus::Cancelled => "EXEC_STATUS_CANCELLED",
            ExecStatus::Completed => "EXEC_STATUS_COMPLETED",
            ExecStatus::Failed => "EXEC_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXEC_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "EXEC_STATUS_STARTING" => Some(Self::Starting),
            "EXEC_STATUS_ABORTED" => Some(Self::Aborted),
            "EXEC_STATUS_CANCELLED" => Some(Self::Cancelled),
            "EXEC_STATUS_COMPLETED" => Some(Self::Completed),
            "EXEC_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}