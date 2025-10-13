use std::sync::atomic::{AtomicI64, Ordering};

use derivative::Derivative;
use futures_util::StreamExt;
use itertools::Itertools;
use tonic::Streaming;
use tracing::{debug, trace};

use ydb_grpc::ydb_proto::query::transaction_settings::TxMode;
use ydb_grpc::ydb_proto::query::v1::query_service_client::QueryServiceClient;
use ydb_grpc::ydb_proto::query::{
    execute_query_request, AttachSessionRequest,
    CommitTransactionRequest as CommitTransactionQueryServiceRequest, DeleteSessionRequest,
    ExecuteQueryRequest, ExecuteQueryResponsePart,
    RollbackTransactionRequest as RollbackTransactionQueryServiceRequest, SessionState, Syntax,
    TransactionControl, TransactionSettings,
};
use ydb_grpc::ydb_proto::table::v1::table_service_client::TableServiceClient;
use ydb_grpc::ydb_proto::table::{
    execute_scan_query_request, ExecuteScanQueryPartialResponse, ExecuteScanQueryRequest,
};
use ydb_grpc::ydb_proto::topic::Codec::Raw;
use ydb_grpc::ydb_proto::TypedValue;

use crate::client::TimeoutSettings;
use crate::client_table::TableServiceClientType;
use crate::errors::{YdbError, YdbResult};
use crate::grpc_connection_manager::GrpcConnectionManager;
use crate::grpc_wrapper::raw_errors::RawResult;
use crate::grpc_wrapper::raw_query_service::begin_transaction::RawBeginTransactionRequest as BeginTransactionQueryServiceRequest;
use crate::grpc_wrapper::raw_query_service::begin_transaction::RawBeginTransactionRequest as RawBeginTransactionQueryServiceRequest;
use crate::grpc_wrapper::raw_query_service::client::{
    RawQueryClient as RawQueryServiceClient, RawQueryClient,
};
use crate::grpc_wrapper::raw_query_service::commit_transaction::RawCommitTransactionRequest as RawCommitTransactionQueryServiceRequest;
use crate::grpc_wrapper::raw_query_service::execute_query::RawExecuteQueryRequest;
use crate::grpc_wrapper::raw_query_service::rollback_transaction::RawRollbackTransactionRequest as RawRollbackTransactionQueryServiceRequest;
use crate::grpc_wrapper::raw_table_service::bulk_upsert::RawBulkUpsertRequest;
use crate::grpc_wrapper::raw_table_service::client::{
    CollectStatsMode, RawTableClient, SessionStatus,
};
use crate::grpc_wrapper::raw_table_service::commit_transaction::RawCommitTransactionRequest as RawCommitTransactionTableRequest;
use crate::grpc_wrapper::raw_table_service::copy_table::{
    RawCopyTableRequest, RawCopyTablesRequest,
};
use crate::grpc_wrapper::raw_table_service::execute_data_query::RawExecuteDataQueryRequest;
use crate::grpc_wrapper::raw_table_service::execute_scheme_query::RawExecuteSchemeQueryRequest;
use crate::grpc_wrapper::raw_table_service::keepalive::RawKeepAliveRequest;
use crate::grpc_wrapper::raw_table_service::rollback_transaction::RawRollbackTransactionRequest as RawRollbackTransactionTableRequest;
use crate::grpc_wrapper::runtime_interceptors::InterceptedChannel;
use crate::result::{QueryResult, StreamQueryResult, StreamResult, StreamTableResult};
use crate::table_service_types::CopyTableItem;
use crate::trace_helpers::ensure_len_string;
use crate::types::Value;
use crate::Query;

static REQUEST_NUMBER: AtomicI64 = AtomicI64::new(0);
static DEFAULT_COLLECT_STAT_MODE: CollectStatsMode = CollectStatsMode::None;

fn req_number() -> i64 {
    REQUEST_NUMBER.fetch_add(1, Ordering::Relaxed)
}

type DropSessionCallback<Client> = dyn FnOnce(&mut Session<Client>) + Send + Sync;

pub(crate) trait SessionInterface<C>
where
    C: Client,
{
    async fn begin_transaction(&mut self, tx_mode: TxMode) -> YdbResult<()>;
    async fn commit_transaction(&mut self, tx_id: String) -> YdbResult<()>;
    async fn rollback_transaction(&mut self, tx_id: String) -> YdbResult<()>;
}

pub(crate) trait Client {
    type Raw;
    type Service;
}

pub struct TableSessionClient;
impl Client for TableSessionClient {
    type Raw = RawTableClient;
    type Service = TableServiceClient<InterceptedChannel>;
}

pub struct QueryServiceSessionClient;
impl Client for QueryServiceSessionClient {
    type Raw = RawQueryClient;
    type Service = QueryServiceClient<InterceptedChannel>;
}

#[derive(Derivative)]
#[derivative(Debug)]
pub(crate) struct Session<C>
where
    C: Client,
{
    pub(crate) id: String,
    pub(crate) can_pooled: bool,

    #[derivative(Debug = "ignore")]
    on_drop_callbacks: Vec<Box<DropSessionCallback<C>>>,

    #[derivative(Debug = "ignore")]
    channel_pool: Box<dyn CreateClient<C>>,

    timeouts: TimeoutSettings,
}

pub(crate) type TableSession = Session<TableSessionClient>;
pub(crate) type QueryServiceSession = Session<QueryServiceSessionClient>;

impl SessionInterface<TableSessionClient> for TableSession {
    async fn begin_transaction(&mut self, _tx_mode: TxMode) -> YdbResult<()> {
        unimplemented!()
    }

    async fn commit_transaction(&mut self, tx_id: String) -> YdbResult<()> {
        let mut table = self.get_client().await?;
        let res = table
            .commit_transaction(RawCommitTransactionTableRequest {
                session_id: self.id.clone(),
                tx_id,
                operation_params: self.timeouts.operation_params(),
                collect_stats: DEFAULT_COLLECT_STAT_MODE,
            })
            .await;
        self.handle_raw_result(res)?;
        Ok(())
    }
    async fn rollback_transaction(&mut self, tx_id: String) -> YdbResult<()> {
        let mut table = self.get_client().await?;
        let res = table
            .rollback_transaction(RawRollbackTransactionTableRequest {
                session_id: self.id.clone(),
                tx_id,
                operation_params: self.timeouts.operation_params(),
            })
            .await;

        self.handle_raw_result(res)
    }
}

impl<C: Client> Session<C> {
    pub(crate) fn new(
        id: String,
        channel_pool: impl CreateClient<C> + 'static,
        timeouts: TimeoutSettings,
    ) -> Self {
        Self {
            id,
            can_pooled: true,
            on_drop_callbacks: Vec::new(),
            channel_pool: Box::new(channel_pool),
            timeouts,
        }
    }

    fn handle_error(&mut self, err: &YdbError) {
        if let YdbError::YdbStatusError(err) = err {
            use ydb_grpc::ydb_proto::status_ids::StatusCode;
            if let Some(status) = StatusCode::from_i32(err.operation_status) {
                if status == StatusCode::BadSession || status == StatusCode::SessionExpired {
                    self.can_pooled = false;
                }
            }
        }
    }

    fn handle_raw_result<T>(&mut self, res: RawResult<T>) -> YdbResult<T> {
        let res = res.map_err(YdbError::from);
        if let Err(err) = &res {
            self.handle_error(err);
        }
        res
    }
    pub(crate) fn clone_without_ondrop(&self) -> Self {
        Self {
            id: self.id.clone(),
            can_pooled: self.can_pooled,
            on_drop_callbacks: Vec::new(),
            channel_pool: self.channel_pool.clone_box(),
            timeouts: self.timeouts,
        }
    }

    // deprecated, use get_table_client instead
    async fn get_channel(&self) -> YdbResult<C::Service> {
        self.channel_pool.create_grpc_client().await
    }

    pub(crate) fn with_timeouts(mut self, timeouts: TimeoutSettings) -> Self {
        self.timeouts = timeouts;
        self
    }

    async fn get_client(&self) -> YdbResult<C::Raw> {
        self.channel_pool.create_client(self.timeouts).await
    }

    #[allow(dead_code)]
    pub(crate) fn on_drop(&mut self, f: Box<dyn FnOnce(&mut Self) + Send + Sync>) {
        self.on_drop_callbacks.push(f)
    }
}

impl TableSession {
    pub(crate) async fn keepalive(&mut self) -> YdbResult<()> {
        let mut table = self.get_client().await?;
        let res = table
            .keep_alive(RawKeepAliveRequest {
                operation_params: self.timeouts.operation_params(),
                session_id: self.id.clone(),
            })
            .await;

        let res = self.handle_raw_result(res)?;

        if let SessionStatus::Ready = res.session_status {
            Ok(())
        } else {
            let err = YdbError::from_str(format!("bad status while session ping: {res:?}"));
            self.handle_error(&err);
            Err(err)
        }
    }

    pub(crate) async fn execute_schema_query(&mut self, query: String) -> YdbResult<()> {
        let res = self
            .get_client()
            .await?
            .execute_scheme_query(RawExecuteSchemeQueryRequest {
                session_id: self.id.clone(),
                yql_text: query,
                operation_params: self.timeouts.operation_params(),
            })
            .await;
        self.handle_raw_result(res)?;
        Ok(())
    }

    pub(crate) async fn execute_bulk_upsert(
        &mut self,
        table_path: String,
        rows: Value,
    ) -> YdbResult<()> {
        let req = RawBulkUpsertRequest {
            table: table_path,
            rows: rows.to_typed_value()?,
            operation_params: self.timeouts.operation_params(),
        };
        let res = self.get_client().await?.bulk_upsert(req).await;
        self.handle_raw_result(res)?;
        Ok(())
    }

    #[tracing::instrument(skip(self, req), fields(req_number=req_number()))]
    pub(crate) async fn execute_data_query(
        &mut self,
        mut req: RawExecuteDataQueryRequest,
        error_on_truncated: bool,
    ) -> YdbResult<QueryResult> {
        req.session_id.clone_from(&self.id);
        req.operation_params = self.timeouts.operation_params();

        trace!(
            "request: {}",
            ensure_len_string(serde_json::to_string(&req)?)
        );

        let res = self.get_client().await?.execute_data_query(req).await;
        let res = self.handle_raw_result(res)?;
        trace!(
            "result: {}",
            ensure_len_string(serde_json::to_string(&res)?)
        );
        if error_on_truncated {
            return Err(YdbError::from_str("result of query was truncated"));
        }
        QueryResult::from_raw_result(error_on_truncated, res)
    }

    #[tracing::instrument(skip(self, query), fields(req_number=req_number()))]
    pub async fn execute_scan_query(&mut self, query: Query) -> YdbResult<StreamTableResult> {
        let params = query.clone().params_to_proto()?;
        let req = ExecuteScanQueryRequest {
            query: Some(query.query_to_table_proto()),
            parameters: params,
            mode: execute_scan_query_request::Mode::Exec as i32,
            ..ExecuteScanQueryRequest::default()
        };
        debug!(
            "request: {}",
            crate::trace_helpers::ensure_len_string(serde_json::to_string(&req)?)
        );
        let mut channel = self.get_channel().await?;
        let resp = channel.stream_execute_scan_query(req).await?;
        let stream = resp.into_inner();
        let res = StreamTableResult { results: stream };
        Ok(res)
    }

    pub async fn copy_table(
        &mut self,
        source_path: String,
        destination_path: String,
    ) -> YdbResult<()> {
        let mut table = self.get_client().await?;
        let res = table
            .copy_table(RawCopyTableRequest {
                session_id: self.id.clone(),
                source_path,
                destination_path,
                operation_params: self.timeouts.operation_params(),
            })
            .await;

        self.handle_raw_result(res)
    }

    pub async fn copy_tables(&mut self, tables: Vec<CopyTableItem>) -> YdbResult<()> {
        let mut table = self.get_client().await?;
        let res = table
            .copy_tables(RawCopyTablesRequest {
                operation_params: self.timeouts.operation_params(),
                session_id: self.id.clone(),
                tables: tables.into_iter().map_into().collect(),
            })
            .await;

        self.handle_raw_result(res)
    }
}

impl SessionInterface<QueryServiceSessionClient> for QueryServiceSession {
    async fn begin_transaction(&mut self, tx_mode: TxMode) -> YdbResult<()> {
        let mut client = self.get_client().await?;
        let res = client
            .begin_transaction(RawBeginTransactionQueryServiceRequest {
                session_id: self.id.clone(),
                settings: TransactionSettings {
                    tx_mode: Some(tx_mode),
                },
            })
            .await;
        self.handle_raw_result(res)?;
        Ok(())
    }

    async fn commit_transaction(&mut self, tx_id: String) -> YdbResult<()> {
        let mut client = self.get_client().await?;
        let res = client
            .commit_transaction(RawCommitTransactionQueryServiceRequest {
                session_id: self.id.clone(),
                tx_id,
            })
            .await;
        self.handle_raw_result(res)?;
        Ok(())
    }
    async fn rollback_transaction(&mut self, tx_id: String) -> YdbResult<()> {
        let mut client = self.get_client().await?;
        let res = client
            .rollback_transaction(RawRollbackTransactionQueryServiceRequest {
                session_id: self.id.clone(),
                tx_id,
            })
            .await;

        self.handle_raw_result(res)
    }
}

impl QueryServiceSession {
    pub async fn attach_session(&mut self) -> YdbResult<Streaming<SessionState>> {
        let mut client = self.get_client().await?;
        let attach_session_req = AttachSessionRequest {
            session_id: self.id.clone(),
        };
        println!("Session id  is attached: {}", self.id);
        let mut stream = client.attach_session(attach_session_req).await?;
        if let Some(res) = stream.next().await {
            match res {
                Ok(part) => {
                    println!("Result is saved!{:?}", part);
                }
                Err(_) => {}
            }
        }
        Ok(stream)
    }

    pub async fn execute_query(&mut self, query: Query) -> YdbResult<StreamQueryResult> {
        let mut client = self.get_client().await?;
        let execute_query_req: ExecuteQueryRequest = ExecuteQueryRequest {
            session_id: self.id.clone(),
            tx_control: query.tx_control.clone(),
            query: Some(query.query_to_query_service_proto(Syntax::YqlV1)),
            parameters: query.params_to_proto()?,
            exec_mode: ydb_grpc::ydb_proto::query::ExecMode::Execute.into(),
            stats_mode: ydb_grpc::ydb_proto::query::StatsMode::Unspecified.into(),
            ..ExecuteQueryRequest::default()
        };
        let mut stream = client.execute_query_stream(execute_query_req).await?;
        let res = StreamQueryResult { results: stream };
        Ok(res)
    }
}

impl<C> Drop for Session<C>
where
    C: Client, // Add this bound
{
    fn drop(&mut self) {
        trace!("drop session: {}", &self.id);
        while let Some(on_drop) = self.on_drop_callbacks.pop() {
            on_drop(self)
        }
    }
}

#[async_trait::async_trait]
pub(crate) trait CreateClient<C>: Send + Sync
where
    C: Client,
{
    async fn create_grpc_client(&self) -> YdbResult<C::Service>;
    async fn create_client(&self, timeouts: TimeoutSettings) -> YdbResult<C::Raw>;
    fn clone_box(&self) -> Box<dyn CreateClient<C>>;
}

#[async_trait::async_trait]
impl CreateClient<TableSessionClient> for GrpcConnectionManager {
    async fn create_grpc_client(&self) -> YdbResult<TableServiceClient<InterceptedChannel>> {
        self.get_auth_service(TableServiceClient::<InterceptedChannel>::new)
            .await
    }

    async fn create_client(&self, timeouts: TimeoutSettings) -> YdbResult<RawTableClient> {
        self.get_auth_service(RawTableClient::new)
            .await
            .map(|item| item.with_timeout(timeouts))
    }

    fn clone_box(&self) -> Box<dyn CreateClient<TableSessionClient>> {
        Box::new(self.clone())
    }
}

#[async_trait::async_trait]
impl CreateClient<QueryServiceSessionClient> for GrpcConnectionManager {
    async fn create_grpc_client(&self) -> YdbResult<QueryServiceClient<InterceptedChannel>> {
        self.get_auth_service(QueryServiceClient::<InterceptedChannel>::new)
            .await
    }

    async fn create_client(&self, timeouts: TimeoutSettings) -> YdbResult<RawQueryClient> {
        self.get_auth_service(RawQueryClient::new)
            .await
            .map(|item| item.with_timeout(timeouts))
    }

    fn clone_box(&self) -> Box<dyn CreateClient<QueryServiceSessionClient>> {
        Box::new(self.clone())
    }
}
