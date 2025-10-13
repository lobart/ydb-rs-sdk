use std::time::Duration;
use tokio::time::timeout;
use ydb::{ydb_params, ClientBuilder, Query, QueryClient, Value, YdbError, YdbResult};
use ydb_grpc::ydb_proto::query::transaction_settings::TxMode;
use ydb_grpc::ydb_proto::query::{SnapshotModeSettings, TransactionSettings};

mod common;
use common::{get_data_for_it_crowd, get_data_for_silicon_valley};
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

    let mut query_client = client.query_service_client();

    create_tables(&mut query_client).await?;
    println!("Tables are created");
    let (series, seasons, episodes) = get_data_for_it_crowd()?;
    fill_tables(&mut query_client, series, seasons, episodes).await?;
    let (series, seasons, episodes) = get_data_for_silicon_valley()?;
    fill_tables(&mut query_client, series, seasons, episodes).await?;
    println!("Tables are filled");
    read(&mut query_client).await?;
    println!("OK");

    Ok(())
}

async fn create_tables(client: &mut QueryClient) -> YdbResult<()> {
    let prefix = "series";
    let query_series = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS {prefix} (
			series_id Bytes,
			title Text,
			series_info Text,
			release_date Date,
			comment Text,

			PRIMARY KEY(series_id));"
    ));
    let prefix = "seasons";
    let query_seasons = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS {prefix} (
			series_id Bytes,
			season_id Bytes,
			title Text,
			first_aired Date,
			last_aired Date,

			PRIMARY KEY(series_id,season_id));"
    ));
    let prefix = "episodes";
    let query_episodes = Query::new(format!(
        "CREATE TABLE IF NOT EXISTS  {prefix} (
			series_id Bytes,
			season_id Bytes,
			episode_id Bytes,
			title Text,
			air_date Date,

			PRIMARY KEY(series_id,season_id,episode_id));"
    ));

    execute_and_print_raws(client, query_series).await?;
    execute_and_print_raws(client, query_seasons).await?;
    execute_and_print_raws(client, query_episodes).await?;

    Ok(())
}

async fn fill_tables(
    client: &mut QueryClient,
    series: Value,
    seasons: Value,
    episodes: Value,
) -> YdbResult<()> {
    let prefix = "`/local/series`";
    let query_fill_series = Query::new(format!(
        "DECLARE $series_list AS List<Struct<
			series_id: Bytes,
			title: Text,
			series_info: Text,
			release_date: Date,
			comment: Text>>;

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
        "$series_list" => series
    ));

    let prefix = "`/local/seasons`";
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
        "$seasons_list" => seasons
    ));

    let prefix = "`/local/episodes`";
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
        "$episodes_list" => episodes
    ));
    execute_and_print_raws(client, query_fill_series).await?;
    execute_and_print_raws(client, query_fill_seasons).await?;
    execute_and_print_raws(client, query_fill_episodes).await?;

    Ok(())
}

async fn read(client: &mut QueryClient) -> YdbResult<()> {
    let table_path = "`/local/series`";
    let commit_tx = true;
    let query = Query::new(format!(
        "SELECT series_id, title, release_date FROM {table_path};"
    ))
    .with_tx_control(
        commit_tx,
        Some(TxSelector::BeginTx(TransactionSettings {
            tx_mode: Some(TxMode::SnapshotReadOnly(SnapshotModeSettings {})),
        })),
    );
    execute_and_print_raws(client, query).await
}

async fn execute_and_print_raws(client: &mut QueryClient, query_series: Query) -> YdbResult<()> {
    while let Some(result) = client
        .execute_query(query_series.clone())
        .await?
        .next()
        .await?
    {
        for row in result.rows() {
            println!("{:?}", row);
        }
    }
    Ok(())
}
