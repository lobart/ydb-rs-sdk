use crate::client::TimeoutSettings;
use crate::grpc_wrapper::raw_errors::RawError;
use crate::grpc_wrapper::raw_errors::RawResult;
use crate::grpc_wrapper::raw_query_service::fetch_script_results::{
    RawFetchScriptResultsRequest, RawFetchScriptResultsResponse,
};
use crate::grpc_wrapper::raw_ydb_operation::RawOperationParams;
use crate::grpc_wrapper::runtime_interceptors::InterceptedChannel;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::client_table::{Retry, TimeoutRetrier};
use crate::errors::NeedRetry;
use crate::grpc_connection_manager::GrpcConnectionManager;
use crate::grpc_wrapper::raw_query_service::execute_query::RawExecuteQueryRequest;
use crate::session_pool::SessionPool;
use crate::transaction::{AutoCommit, SerializableReadWriteTx};
use crate::{Mode, Query, StreamResult, Transaction, TransactionOptions, YdbResult};
use futures_util::StreamExt;
use ydb_grpc::ydb_proto::query;
use ydb_grpc::ydb_proto::query::{QueryContent, SerializableModeSettings, Syntax};

#[derive(Clone)]
pub struct QueryClient {
    error_on_truncate: bool,
    session_pool: SessionPool,
    retrier: Arc<Box<dyn Retry>>,
    transaction_options: TransactionOptions,
    idempotent_operation: bool,
    timeouts: TimeoutSettings,
}

impl QueryClient {
    pub(crate) fn new(
        connection_manager: GrpcConnectionManager,
        timeouts: TimeoutSettings,
    ) -> Self {
        Self {
            error_on_truncate: false,
            session_pool: SessionPool::new(Box::new(connection_manager), timeouts),
            retrier: Arc::new(Box::<crate::client_table::TimeoutRetrier>::default()),
            transaction_options: TransactionOptions::new(),
            idempotent_operation: false,
            timeouts,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_max_active_sessions(mut self, size: usize) -> Self {
        self.session_pool = self.session_pool.with_max_active_sessions(size);
        self
    }

    // Clone the table client and set new timeouts settings
    pub fn clone_with_timeouts(&self, timeouts: TimeoutSettings) -> Self {
        Self {
            timeouts,
            ..self.clone()
        }
    }

    /// Clone the table client and set new retry timeouts
    #[allow(dead_code)]
    pub fn clone_with_retry_timeout(&self, timeout: Duration) -> Self {
        Self {
            retrier: Arc::new(Box::new(TimeoutRetrier { timeout })),
            ..self.clone()
        }
    }

    /// Clone the table client and deny retries
    #[allow(dead_code)]
    pub fn clone_with_no_retry(&self) -> Self {
        Self {
            retrier: Arc::new(Box::new(crate::client_table::NoRetrier {})),
            ..self.clone()
        }
    }

    /// Clone the table client and set feature operations as idempotent (can retry in more cases)
    #[allow(dead_code)]
    pub fn clone_with_idempotent_operations(&self, idempotent: bool) -> Self {
        Self {
            idempotent_operation: idempotent,
            ..self.clone()
        }
    }

    pub fn clone_with_transaction_options(&self, opts: TransactionOptions) -> Self {
        Self {
            transaction_options: opts,
            ..self.clone()
        }
    }

    pub(crate) fn create_autocommit_transaction(&self, mode: Mode) -> impl Transaction {
        AutoCommit::new(self.session_pool.clone(), mode, self.timeouts)
            .with_error_on_truncate(self.error_on_truncate)
    }

    pub(crate) fn create_interactive_transaction(&self) -> impl Transaction {
        SerializableReadWriteTx::new(self.session_pool.clone(), self.timeouts)
            .with_error_on_truncate(self.error_on_truncate)
    }

    #[allow(dead_code)]
    pub async fn create_session(&self) -> YdbResult<SessionSt> {
        Ok(self
            .session_pool
            .session()
            .await?
            .with_timeouts(self.timeouts))
    }

    async fn retry<CallbackFuture, CallbackResult>(
        &self,
        callback: impl Fn() -> CallbackFuture,
    ) -> YdbResult<CallbackResult>
    where
        CallbackFuture: Future<Output = YdbResult<CallbackResult>>,
    {
        let mut attempt: usize = 0;
        let start = Instant::now();
        loop {
            attempt += 1;
            let last_err = match callback().await {
                Ok(res) => return Ok(res),
                Err(err) => match (err.need_retry(), self.idempotent_operation) {
                    (NeedRetry::True, _) => err,
                    (NeedRetry::IdempotentOnly, true) => err,
                    _ => return Err(err),
                },
            };

            let now = std::time::Instant::now();
            let retry_decision = self
                .retrier
                .wait_duration(crate::client_table::RetryParams {
                    attempt,
                    time_from_start: now.duration_since(start),
                });
            if !retry_decision.allow_retry {
                return Err(last_err);
            }
            tokio::time::sleep(retry_decision.wait_timeout).await;
        }
    }

    /// Execute scan query. The method will auto-retry errors while start query execution,
    /// but no retries after server start streaming result.
    pub async fn retry_execute_scan_query(&self, query: Query) -> YdbResult<StreamResult> {
        self.retry(|| async {
            let mut session = self.create_session().await?;
            session.execute_scan_query(query.clone()).await
        })
        .await
    }

    /// Execute scheme query with retry policy
    pub async fn retry_execute_scheme_query<T: Into<String>>(&self, query: T) -> YdbResult<()> {
        let query = Arc::new(query.into());
        self.retry(|| async {
            let mut session = self.create_session().await?;
            session.execute_schema_query(query.to_string()).await
        })
        .await
    }
    ///////////////////
    ///////////////////

    /// Execute YQL text in a short-lived transaction and collect all response parts.
    pub async fn execute_yql_collect(
        &mut self,
        yql_text: impl Into<String>,
    ) -> RawResult<Vec<query::ExecuteQueryResponsePart>> {
        let yql = yql_text.into();
        let req = query::ExecuteQueryRequest {
            session_id: "".to_string(),
            query: Some(query::execute_query_request::Query::QueryContent(
                query::QueryContent {
                    syntax: Syntax::YqlV1.into(),
                    text: yql_text.clone(),
                },
            )),
            // Default: begin + commit short-living transaction (SerializableRW)
            tx_control: Some(query::TransactionControl {
                tx_selector: Some(query::transaction_control::TxSelector::BeginTx(
                    query::TransactionSettings {
                        tx_mode: Some(query::transaction_settings::TxMode::SerializableReadWrite(
                            SerializableModeSettings {},
                        )),
                    },
                )),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut stream = self
            .raw
            .execute_query_stream(RawExecuteQueryRequest(req))
            .await?;
        let mut parts = Vec::new();
        while let Some(next) = stream.next().await {
            parts.push(next?);
        }
        Ok(parts)
    }

    // /// Kick off a long-running script via Operation API (no payload result).
    // pub async fn execute_script(
    //     &mut self,
    //     req: query::ExecuteScriptRequest,
    //     operation_timeouts: RawOperationParams,
    // ) -> RawResult<()> {
    //     let raw = RawExecuteScriptRequest { operation_params: operation_timeouts, request: req };
    //     self.raw.execute_script(raw).await
    // }
    //
    //
    // pub async fn fetch_script_results(
    //     &mut self,
    //     req: query::FetchScriptResultsRequest,
    // ) -> RawResult<RawFetchScriptResultsResponse> {
    //     self.raw.fetch_script_results(RawFetchScriptResultsRequest(req)).await
    // }

    // pub async fn create_session(&mut self) -> RawResult<RawCreateSessionResult> {
    //     self.raw.create_session(RawCreateSessionRequest::default()).await
    // }
    // pub async fn delete_session(&mut self, session_id: String) -> RawResult<()> {
    //     self.raw.delete_session(RawDeleteSessionRequest { session_id }).await
    // }
    // pub async fn begin_transaction(&mut self, session_id: String, settings: query::TransactionSettings) -> RawResult<RawBeginTransactionResult> {
    //     self.raw.begin_transaction(RawBeginTransactionRequest { session_id, settings }).await
    // }
    // pub async fn commit_transaction(&mut self, session_id: String, tx_id: String) -> RawResult<RawCommitTransactionResult> {
    //     self.raw.commit_transaction(RawCommitTransactionRequest { session_id, tx_id }).await
    // }
    // pub async fn rollback_transaction(&mut self, session_id: String, tx_id: String) -> RawResult<()> {
    //     self.raw.rollback_transaction(RawRollbackTransactionRequest { session_id, tx_id }).await
    // }
}
