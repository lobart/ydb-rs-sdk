pub(crate) struct RawAttachSessionRequest {
    pub session_id: String,
}

impl From<RawAttachSessionRequest> for ydb_grpc::ydb_proto::query::AttachSessionRequest {
    fn from(r: RawAttachSessionRequest) -> Self {
        Self {
            session_id: r.session_id,
        }
    }
}
