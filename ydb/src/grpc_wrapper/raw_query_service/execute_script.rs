use crate::grpc_wrapper::raw_ydb_operation::RawOperationParams;
use ydb_grpc::ydb_proto::{operations::OperationParams, query::ExecuteScriptRequest};

#[derive(Debug)]
pub(crate) struct RawExecuteScriptRequest {
    pub operation_params: RawOperationParams,
    pub request: ExecuteScriptRequest,
}

impl From<RawExecuteScriptRequest> for ExecuteScriptRequest {
    fn from(r: RawExecuteScriptRequest) -> Self {
        let mut req = r.request;
        req.operation_params = Some(OperationParams::from(r.operation_params));
        req
    }
}
