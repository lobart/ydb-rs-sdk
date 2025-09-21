use crate::grpc_wrapper::raw_errors::{RawError, RawResult};
use ydb_grpc::ydb_proto::query::{
    BeginTransactionRequest, BeginTransactionResponse, TransactionSettings,
};

#[derive(Debug)]
pub(crate) struct RawBeginTransactionRequest {
    pub session_id: String,
    pub settings: TransactionSettings,
}
impl From<RawBeginTransactionRequest> for BeginTransactionRequest {
    fn from(r: RawBeginTransactionRequest) -> Self {
        BeginTransactionRequest {
            session_id: r.session_id,
            tx_settings: Some(r.settings),
        }
    }
}

#[derive(Debug)]
pub(crate) struct RawBeginTransactionResult {
    pub tx_id: String,
}
impl TryFrom<BeginTransactionResponse> for RawBeginTransactionResult {
    type Error = RawError;
    fn try_from(resp: BeginTransactionResponse) -> RawResult<Self> {
        let id = resp.tx_meta.map(|m| m.id).unwrap_or_default();
        if id.is_empty() {
            return Err(RawError::Custom("empty tx_id".into()));
        }
        Ok(Self { tx_id: id })
    }
}
