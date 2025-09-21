use crate::grpc_wrapper::raw_errors::RawError;
use ydb_grpc::ydb_proto::query::{
    BeginTransactionRequest, BeginTransactionResponse, CommitTransactionRequest,
    CommitTransactionResponse, RollbackTransactionRequest, RollbackTransactionResponse,
};

#[derive(Debug)]
pub(crate) struct RawBeginTransactionRequest(pub BeginTransactionRequest);
impl From<RawBeginTransactionRequest> for BeginTransactionRequest {
    fn from(r: RawBeginTransactionRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawBeginTransactionResponse(pub BeginTransactionResponse);
impl TryFrom<BeginTransactionResponse> for RawBeginTransactionResponse {
    type Error = RawError;
    fn try_from(value: BeginTransactionResponse) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Debug)]
pub(crate) struct RawCommitTransactionRequest(pub CommitTransactionRequest);
impl From<RawCommitTransactionRequest> for CommitTransactionRequest {
    fn from(r: RawCommitTransactionRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawCommitTransactionResponse(pub CommitTransactionResponse);
impl TryFrom<CommitTransactionResponse> for RawCommitTransactionResponse {
    type Error = RawError;
    fn try_from(value: CommitTransactionResponse) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Debug)]
pub(crate) struct RawRollbackTransactionRequest(pub RollbackTransactionRequest);
impl From<RawRollbackTransactionRequest> for RollbackTransactionRequest {
    fn from(r: RawRollbackTransactionRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawRollbackTransactionResponse(pub RollbackTransactionResponse);
impl TryFrom<RollbackTransactionResponse> for RawRollbackTransactionResponse {
    type Error = RawError;
    fn try_from(value: RollbackTransactionResponse) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
