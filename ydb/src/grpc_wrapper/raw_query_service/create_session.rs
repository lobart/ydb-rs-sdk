use crate::grpc_wrapper::raw_errors::{RawError, RawResult};
use ydb_grpc::ydb_proto::query::{CreateSessionRequest, CreateSessionResponse};

#[derive(Debug, Default)]
pub(crate) struct RawCreateSessionRequest; // no fields; Query CreateSession takes no params

impl From<RawCreateSessionRequest> for CreateSessionRequest {
    fn from(_: RawCreateSessionRequest) -> Self {
        CreateSessionRequest::default()
    }
}

#[derive(Debug)]
pub(crate) struct RawCreateSessionResult {
    pub session_id: String,
}

impl TryFrom<CreateSessionResponse> for RawCreateSessionResult {
    type Error = RawError;
    fn try_from(resp: CreateSessionResponse) -> RawResult<Self> {
        let sid = resp.session_id;
        if sid.is_empty() {
            return Err(RawError::Custom("empty session_id".into()));
        }
        Ok(Self { session_id: sid })
    }
}
