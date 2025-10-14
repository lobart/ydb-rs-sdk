use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use tonic::Streaming;

use ydb_grpc::ydb_proto::query::transaction_settings::TxMode;
use ydb_grpc::ydb_proto::query::SessionState;

use crate::client::TimeoutSettings;
use crate::retrier::{Retry, TimeoutRetrier};
use crate::errors::NeedRetry;
use crate::grpc_connection_manager::GrpcConnectionManager;
use crate::result::StreamQueryResult;
use crate::session::{QueryServiceSession, SessionInterface};
use crate::session_pool::SessionPool;
use crate::{Query, Transaction, TransactionOptions, YdbResult};

#[derive(Clone)]
pub struct QueryClient {
    error_on_truncate: bool,
    session_pool: SessionPool<QueryServiceSession>,
    active_session: Option<Arc<RwLock<(QueryServiceSession, Streaming<SessionState>)>>>,
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
            session_pool: SessionPool::<QueryServiceSession>::new(
                Box::new(connection_manager),
                timeouts,
            ),
            active_session: None,
            retrier: Arc::new(Box::<crate::retrier::TimeoutRetrier>::default()),
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
            retrier: Arc::new(Box::new(crate::retrier::NoRetrier {})),
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

    #[allow(dead_code)]
    pub async fn create_session(&self) -> YdbResult<QueryServiceSession> {
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
                .wait_duration(crate::retrier::RetryParams {
                    attempt,
                    time_from_start: now.duration_since(start),
                });
            if !retry_decision.allow_retry {
                return Err(last_err);
            }
            tokio::time::sleep(retry_decision.wait_timeout).await;
        }
    }

    pub async fn commit_transaction(&mut self, tx_id: String) -> YdbResult<()> {
        let mut session = self.create_session().await?;
        let stream_attached = session.attach_session().await?;
        session.commit_transaction(tx_id).await?;
        Ok(())
    }

    pub async fn begin_transaction(&mut self, tx_mode: TxMode) -> YdbResult<()> {
        let mut session = self.create_session().await?;
        let stream_attached = session.attach_session().await?;
        session.begin_transaction(tx_mode).await?;
        Ok(())
    }

    pub async fn execute_query(&mut self, query: Query) -> YdbResult<StreamQueryResult> {
        let mut session = if let Some(active_session) = &self.active_session {
            active_session.clone()
        } else {
            let mut new_session = self.create_session().await?;
            let stream_attached = new_session.attach_session().await?;
            Arc::new(RwLock::new((new_session, stream_attached)))
        };
        self.active_session = Some(session.clone());
        let res = (*session.write().await).0.execute_query(query).await?;
        Ok(res)
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
