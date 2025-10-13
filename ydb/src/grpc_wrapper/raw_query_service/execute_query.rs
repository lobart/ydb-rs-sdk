use crate::grpc_wrapper::raw_errors::RawError;
use ydb_grpc::ydb_proto::query;

#[derive(Debug)]
pub(crate) struct RawExecuteQueryRequest(pub query::ExecuteQueryRequest);
impl From<RawExecuteQueryRequest> for query::ExecuteQueryRequest {
    fn from(r: RawExecuteQueryRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawExecuteQueryResponsePart(pub query::ExecuteQueryResponsePart);
impl TryFrom<query::ExecuteQueryResponsePart> for RawExecuteQueryResponsePart {
    type Error = RawError;
    fn try_from(value: query::ExecuteQueryResponsePart) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
