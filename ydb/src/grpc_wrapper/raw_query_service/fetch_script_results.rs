use crate::grpc_wrapper::raw_errors::RawError;
use ydb_grpc::ydb_proto::query::{FetchScriptResultsRequest, FetchScriptResultsResponse};

#[derive(Debug)]
pub(crate) struct RawFetchScriptResultsRequest(pub FetchScriptResultsRequest);
impl From<RawFetchScriptResultsRequest> for FetchScriptResultsRequest {
    fn from(r: RawFetchScriptResultsRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawFetchScriptResultsResponse(pub FetchScriptResultsResponse);
impl TryFrom<FetchScriptResultsResponse> for RawFetchScriptResultsResponse {
    type Error = RawError;
    fn try_from(value: FetchScriptResultsResponse) -> Result<Self, Self::Error> {
        // If you want strict status/issue handling, add checks here similar to other wrappers.
        Ok(Self(value))
    }
}
