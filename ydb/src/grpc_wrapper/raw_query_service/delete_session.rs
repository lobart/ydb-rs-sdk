use crate::grpc_wrapper::raw_errors::RawError;
use ydb_grpc::ydb_proto::query::{DeleteSessionRequest, DeleteSessionResponse};

#[derive(Debug)]
pub(crate) struct RawDeleteSessionRequest(pub DeleteSessionRequest);
impl From<RawDeleteSessionRequest> for DeleteSessionRequest {
    fn from(r: RawDeleteSessionRequest) -> Self {
        r.0
    }
}

#[derive(Debug)]
pub(crate) struct RawDeleteSessionResponse(pub DeleteSessionResponse);
impl TryFrom<DeleteSessionResponse> for RawDeleteSessionResponse {
    type Error = RawError;
    fn try_from(value: DeleteSessionResponse) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
