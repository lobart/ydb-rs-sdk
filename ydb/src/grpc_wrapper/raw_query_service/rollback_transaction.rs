use ydb_grpc::ydb_proto::query::RollbackTransactionRequest;

#[derive(Debug)]
pub(crate) struct RawRollbackTransactionRequest {
    pub session_id: String,
    pub tx_id: String,
}
impl From<RawRollbackTransactionRequest> for RollbackTransactionRequest {
    fn from(r: RawRollbackTransactionRequest) -> Self {
        RollbackTransactionRequest {
            session_id: r.session_id,
            tx_id: r.tx_id,
        }
    }
}
