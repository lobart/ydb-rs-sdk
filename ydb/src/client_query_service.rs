use crate::grpc_wrapper::raw_errors::RawError;
use crate::grpc_wrapper::raw_query_service::fetch_script_results::{
    RawFetchScriptResultsRequest, RawFetchScriptResultsResponse,
};
use crate::grpc_wrapper::raw_ydb_operation::RawOperationParams;
use crate::grpc_wrapper::runtime_interceptors::InterceptedChannel;
use crate::grpc_wrapper::raw_errors::RawResult;
use crate::client::TimeoutSettings;


use futures_util::StreamExt;
use ydb_grpc::ydb_proto::query;

pub struct QueryClient {
    raw: RawQueryClient,
}


impl QueryClient {
    pub fn new(channel: InterceptedChannel) -> Self {
        Self { raw: RawQueryClient::new(channel) }
    }


    pub fn with_timeout(mut self, timeouts: TimeoutSettings) -> Self {
        self.raw = self.raw.with_timeout(timeouts);
        self
    }


    /// Execute YQL text in a short-lived transaction and collect all response parts.
    pub async fn execute_yql_collect(
        &mut self,
        yql_text: impl Into<String>,
    ) -> RawResult<Vec<query::ExecuteQueryResponsePart>> {
        let yql = yql_text.into();
        let req = query::ExecuteQueryRequest {
            query: Some(query::Query{ query: Some(query::query::Query::YqlText(yql)) }),
            // Default: begin + commit short-living transaction (SerializableRW)
            tx_control: Some(query::TransactionControl {
                tx_selector: Some(query::transaction_control::TxSelector::BeginTx(
                    query::TransactionSettings {
                        tx_mode: Some(query::transaction_settings::TxMode::SerializableReadWrite(
                            query::SerializableReadWriteSettings { ..Default::default() }
                        )),
                    }
                )),
                commit_tx: true,
            }),
            ..Default::default()
        };
        let mut stream = self.raw.execute_query_stream(RawExecuteQueryRequest(req)).await?;
        let mut parts = Vec::new();
        while let Some(next) = stream.next().await { parts.push(next?); }
        Ok(parts)
    }


    /// Kick off a long-running script via Operation API (no payload result).
    pub async fn execute_script(
        &mut self,
        req: query::ExecuteScriptRequest,
        operation_timeouts: RawOperationParams,
    ) -> RawResult<()> {
        let raw = RawExecuteScriptRequest { operation_params: operation_timeouts, request: req };
        self.raw.execute_script(raw).await
    }


    pub async fn fetch_script_results(
        &mut self,
        req: query::FetchScriptResultsRequest,
    ) -> RawResult<RawFetchScriptResultsResponse> {
        self.raw.fetch_script_results(RawFetchScriptResultsRequest(req)).await
    }

    pub async fn create_session(&mut self) -> RawResult<RawCreateSessionResult> {
        self.raw.create_session(RawCreateSessionRequest::default()).await
    }
    pub async fn delete_session(&mut self, session_id: String) -> RawResult<()> {
        self.raw.delete_session(RawDeleteSessionRequest { session_id }).await
    }
    pub async fn begin_transaction(&mut self, session_id: String, settings: query::TransactionSettings) -> RawResult<RawBeginTransactionResult> {
        self.raw.begin_transaction(RawBeginTransactionRequest { session_id, settings }).await
    }
    pub async fn commit_transaction(&mut self, session_id: String, tx_id: String) -> RawResult<RawCommitTransactionResult> {
        self.raw.commit_transaction(RawCommitTransactionRequest { session_id, tx_id }).await
    }
    pub async fn rollback_transaction(&mut self, session_id: String, tx_id: String) -> RawResult<()> {
        self.raw.rollback_transaction(RawRollbackTransactionRequest { session_id, tx_id }).await
    }
}