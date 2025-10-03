use std::time::Duration;
use tokio::time::timeout;
use ydb::{
    ydb_params, ydb_struct, Client, ClientBuilder, Query, QueryClient, Value, YdbError, YdbResult,
};
use ydb_grpc::ydb_proto::query::transaction_settings::TxMode;
use ydb_grpc::ydb_proto::query::TransactionSettings;
use ydb_grpc::ydb_proto::query::{ExecuteQueryRequest, SnapshotModeSettings, TransactionControl};

mod data;
use data::{create_value_list, get_data};
use ydb_grpc::ydb_proto::query::transaction_control::TxSelector;

#[tokio::main]
async fn main() -> YdbResult<()> {
    let client = ClientBuilder::new_from_connection_string("grpc://localhost:2136?database=local")?
        .client()?;

    if let Ok(res) = timeout(Duration::from_secs(3), client.wait()).await {
        res?
    } else {
        return Err(YdbError::from("Connection timeout"));
    };

    let table_name = "query";

    let mut query_client = client.query_service_client();

    create_tables(&mut query_client, table_name).await?;
    //fill_tables(&mut query_client, table_name).await?;
    //read(&mut query_client, table_name).await?;

    println!("OK");

    Ok(())
}

async fn create_tables(client: &mut QueryClient, prefix: &str) -> YdbResult<()> {
    let query_series = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS {prefix} (
			series_id Bytes,
			title Text,
			series_info Text,
			release_date Date,
			comment Text,

			PRIMARY KEY(series_id));"
    ))
    .with_tx_control(
        false,
        Some(TxSelector::BeginTx(TransactionSettings {
            tx_mode: Some(TxMode::SnapshotReadOnly(SnapshotModeSettings {})),
        })),
    );

    let query_seasons = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS {prefix} (
			series_id Bytes,
			season_id Bytes,
			title Text,
			first_aired Date,
			last_aired Date,

			PRIMARY KEY(series_id,season_id));"
    ))
    .with_tx_control(
        false,
        Some(TxSelector::BeginTx(TransactionSettings {
            tx_mode: Some(TxMode::SnapshotReadOnly(SnapshotModeSettings {})),
        })),
    );
    let query_episodes = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS  {prefix} (
			series_id Bytes,
			season_id Bytes,
			episode_id Bytes,
			title Text,
			air_date Date,

			PRIMARY KEY(series_id,season_id,episode_id));"
    ))
    .with_tx_control(
        false,
        Some(TxSelector::BeginTx(TransactionSettings {
            tx_mode: Some(TxMode::SnapshotReadOnly(SnapshotModeSettings {})),
        })),
    );

    client.execute_query(query_series).await?;
    client.execute_query(query_seasons).await?;
    client.execute_query(query_episodes).await?;

    Ok(())
}

async fn fill_tables(client: &mut QueryClient, prefix: &str) -> YdbResult<()> {
    let (series, seasons, episodes) = get_data();

    let series_list = create_value_list(series)?;
    let seasons_list = create_value_list(seasons)?;
    let episodes_list = create_value_list(episodes)?;

    let query_fill_series = Query::new(format!(
        "DECLARE $series_list AS List<Struct<
			series_id: Bytes,
			title: Text,
			series_info: Text,
			release_date: Date,
			comment: Optional<Text>>>;

		REPLACE INTO {prefix}
		SELECT
			series_id,
			title,
			series_info,
			release_date,
			comment
		FROM AS_TABLE($series_list);"
    ))
    .with_params(ydb_params!(
        "$series_list" => series_list
    ));

    let query_fill_seasons = Query::new(format!(
        "DECLARE $seasons_list AS List<Struct<
			series_id: Bytes,
			season_id: Bytes,
			title: Text,
			first_aired: Date,
			last_aired: Date>>;

		REPLACE INTO {prefix}
		SELECT
			series_id,
			season_id,
			title,
			first_aired,
			last_aired
		FROM AS_TABLE($seasons_list);"
    ))
    .with_params(ydb_params!(
        "$seasons_list" => seasons_list
    ));

    let query_fill_episodes = Query::new(format!(
        "DECLARE $episodes_list AS List<Struct<
			series_id: Bytes,
			season_id: Bytes,
			episode_id: Bytes,
			title: Text,
			air_date: Date>>;

		REPLACE INTO {prefix}
		SELECT
			series_id,
			season_id,
			episode_id,
			title,
			air_date
		FROM AS_TABLE($episodes_list);"
    ))
    .with_params(ydb_params!(
        "$episodes_list" => episodes_list
    ));
    client.execute_query(query_fill_series).await?;
    client.execute_query(query_fill_seasons).await?;
    client.execute_query(query_fill_episodes).await?;

    Ok(())
}

async fn read(client: &mut QueryClient, prefix: &str) -> YdbResult<()> {
    let table_path = "series";
    let commit_tx = false;
    let query = Query::new(format!(
        "SELECT series_id, title, release_date FROM `{table_path}`"
    ))
    .with_tx_control(
        commit_tx,
        Some(TxSelector::BeginTx(TransactionSettings {
            tx_mode: Some(TxMode::SnapshotReadOnly(SnapshotModeSettings {})),
        })),
    );
    // Execute query with snapshot read-only transaction
    let result = client.execute_query(query).await?;

    Ok(())
}
