use crate::grpc_wrapper::raw_errors::{RawError, RawResult};
use ydb_grpc::generated::ydb::query::{CommitTransactionRequest, CommitTransactionResponse};

#[derive(Debug)]
pub(crate) struct RawCommitTransactionRequest {
    pub session_id: String,
    pub tx_id: String,
}
impl From<RawCommitTransactionRequest> for CommitTransactionRequest {
    fn from(r: RawCommitTransactionRequest) -> Self {
        CommitTransactionRequest {
            session_id: r.session_id,
            tx_id: r.tx_id,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RawCommitTransactionResult;
impl TryFrom<CommitTransactionResponse> for RawCommitTransactionResult {
    type Error = RawError;
    fn try_from(_resp: CommitTransactionResponse) -> RawResult<Self> {
        Ok(Self)
    }
}
