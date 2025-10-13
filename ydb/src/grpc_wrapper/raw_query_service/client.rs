use crate::grpc_wrapper::raw_errors::RawError;
use ydb_grpc::ydb_proto::query::{
    AttachSessionRequest, DeleteSessionRequest, ExecuteQueryRequest, FetchScriptResultsRequest,
    FetchScriptResultsResponse,
};

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
        Ok(Self(value))
    }
}
use crate::client::TimeoutSettings;
use crate::grpc_wrapper::raw_errors::RawResult;
use crate::grpc_wrapper::raw_services::{GrpcServiceForDiscovery, Service};
use crate::grpc_wrapper::runtime_interceptors::InterceptedChannel;

use super::begin_transaction::{RawBeginTransactionRequest, RawBeginTransactionResult};
use super::commit_transaction::{RawCommitTransactionRequest, RawCommitTransactionResult};
use super::delete_session::RawDeleteSessionRequest;
use super::execute_query::RawExecuteQueryRequest;
use super::execute_script::RawExecuteScriptRequest;
use super::rollback_transaction::RawRollbackTransactionRequest;
use crate::grpc_wrapper::raw_query_service::create_session::{
    RawCreateSessionRequest, RawCreateSessionResult as QueryCreateSessionResult,
};
use ydb_grpc::ydb_proto::query;
use ydb_grpc::ydb_proto::query::v1::query_service_client::QueryServiceClient;

pub(crate) struct RawQueryClient {
    timeouts: TimeoutSettings,
    service: QueryServiceClient<InterceptedChannel>,
}

impl RawQueryClient {
    pub fn new(service: InterceptedChannel) -> Self {
        Self {
            service: QueryServiceClient::new(service),
            timeouts: TimeoutSettings::default(),
        }
    }
    pub fn with_timeout(mut self, timeouts: TimeoutSettings) -> Self {
        self.timeouts = timeouts;
        self
    }

    pub async fn create_session(
        &mut self,
        req: RawCreateSessionRequest,
    ) -> RawResult<QueryCreateSessionResult> {
        let grpc_req: query::CreateSessionRequest = req.into();
        let resp = self.service.create_session(grpc_req).await?;
        let inner = resp.into_inner();
        let out: QueryCreateSessionResult = QueryCreateSessionResult::try_from(inner)?; // disambiguated
        Ok(out)
    }

    pub async fn delete_session(&mut self, req: DeleteSessionRequest) -> RawResult<()> {
        let grpc_req: query::DeleteSessionRequest = req.into();
        let _ = self.service.delete_session(grpc_req).await?;
        Ok(())
    }

    pub async fn begin_transaction(
        &mut self,
        req: RawBeginTransactionRequest,
    ) -> RawResult<RawBeginTransactionResult> {
        let grpc_req: query::BeginTransactionRequest = req.into();
        let resp = self.service.begin_transaction(grpc_req).await?;
        let inner = resp.into_inner();
        RawBeginTransactionResult::try_from(inner)
    }

    pub async fn commit_transaction(
        &mut self,
        req: RawCommitTransactionRequest,
    ) -> RawResult<RawCommitTransactionResult> {
        let grpc_req: query::CommitTransactionRequest = req.into();
        let resp = self.service.commit_transaction(grpc_req).await?;
        let inner = resp.into_inner();
        RawCommitTransactionResult::try_from(inner)
    }

    pub async fn rollback_transaction(
        &mut self,
        req: RawRollbackTransactionRequest,
    ) -> RawResult<()> {
        let grpc_req: query::RollbackTransactionRequest = req.into();
        let _ = self.service.rollback_transaction(grpc_req).await?;
        Ok(())
    }

    pub async fn execute_query_stream(
        &mut self,
        req: ExecuteQueryRequest,
    ) -> RawResult<tonic::Streaming<query::ExecuteQueryResponsePart>> {
        let resp = self.service.execute_query(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn attach_session(
        &mut self,
        req: AttachSessionRequest,
    ) -> RawResult<tonic::Streaming<query::SessionState>> {
        Ok(self.service.attach_session(req).await?.into_inner())
    }

    pub async fn execute_script(&mut self, req: RawExecuteScriptRequest) -> RawResult<()> {
        let grpc_req: query::ExecuteScriptRequest = req.into();
        let _ = self.service.execute_script(grpc_req).await?;
        Ok(())
    }

    pub async fn fetch_script_results(
        &mut self,
        req: RawFetchScriptResultsRequest,
    ) -> RawResult<RawFetchScriptResultsResponse> {
        let grpc_req: query::FetchScriptResultsRequest = req.into();
        let resp = self.service.fetch_script_results(grpc_req).await?;
        let inner = resp.into_inner();
        RawFetchScriptResultsResponse::try_from(inner)
    }
}

impl GrpcServiceForDiscovery for RawQueryClient {
    fn get_grpc_discovery_service() -> Service {
        Service::Query
    }
}
