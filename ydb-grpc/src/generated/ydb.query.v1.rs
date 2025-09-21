/// Generated client implementations.
pub mod query_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct QueryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Sessions are basic primitives for communicating with YDB Query Service. The are similar to
        /// connections for classic relational DBs. Sessions serve three main purposes:
        /// 1. Provide a flow control for DB requests with limited number of active channels.
        /// 2. Distribute load evenly across multiple DB nodes.
        /// 3. Store state for volatile stateful operations, such as short-living transactions.
        pub async fn create_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CreateSessionRequest>,
        ) -> Result<
            tonic::Response<super::super::CreateSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/CreateSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::DeleteSessionRequest>,
        ) -> Result<
            tonic::Response<super::super::DeleteSessionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/DeleteSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn attach_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::AttachSessionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::SessionState>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/AttachSession",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Short-living transactions allow transactional execution of several queries, including support
        /// for interactive transactions. Transaction control can be implemented via flags in ExecuteQuery
        /// call (recommended), or via explicit calls to Begin/Commit/RollbackTransaction.
        pub async fn begin_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::BeginTransactionRequest>,
        ) -> Result<
            tonic::Response<super::super::BeginTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/BeginTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn commit_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::CommitTransactionRequest>,
        ) -> Result<
            tonic::Response<super::super::CommitTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/CommitTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn rollback_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::RollbackTransactionRequest>,
        ) -> Result<
            tonic::Response<super::super::RollbackTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/RollbackTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Execute interactive query in a specified short-living transaction.
        /// YDB query can contain DML, DDL and DCL statements. Supported mix of different statement types depends
        /// on the chosen transaction type.
        /// In case of error, including transport errors such as interrupted stream, whole transaction
        /// needs to be retried. For non-idempotent transaction, a custom client logic is required to
        /// retry conditionally retriable statuses, when transaction execution state is unknown.
        pub async fn execute_query(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteQueryRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::ExecuteQueryResponsePart>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/ExecuteQuery",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Execute long-running script.
        /// YDB scripts can contain all type of statements, including TCL statements. This way you can execute multiple
        /// transactions in a single YDB script.
        /// ExecuteScript call returns long-running Ydb.Operation object with:
        ///   operation.metadata = ExecuteScriptMetadata
        ///   operation.result = Empty
        /// Script execution metadata contains all information about current execution state, including
        /// execution_id, execution statistics and result sets info.
        /// You can use standard operation methods such as Get/Cancel/Forget/ListOperations to work with script executions.
        /// Script can be executed as persistent, in which case all execution information and results will be stored
        /// persistently and available after successful or unsuccessful execution.
        pub async fn execute_script(
            &mut self,
            request: impl tonic::IntoRequest<super::super::ExecuteScriptRequest>,
        ) -> Result<
            tonic::Response<super::super::super::operations::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/ExecuteScript",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetch results for script execution using fetch_token for continuation.
        /// For script with multiple result sets, parts of different results sets are interleaved in responses.
        /// For persistent scripts, you can fetch results in specific position of specific result set using
        /// position instead of fetch_token.
        pub async fn fetch_script_results(
            &mut self,
            request: impl tonic::IntoRequest<super::super::FetchScriptResultsRequest>,
        ) -> Result<
            tonic::Response<super::super::FetchScriptResultsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/Ydb.Query.V1.QueryService/FetchScriptResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}