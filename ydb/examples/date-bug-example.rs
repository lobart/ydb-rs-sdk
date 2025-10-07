use std::time::{Duration, SystemTime};
use tokio::time::timeout;
use uuid::Uuid;
use ydb::{ydb_params, ydb_struct, Bytes, ClientBuilder, Query, Value, YdbError, YdbResult};

#[tokio::main]
async fn main() -> YdbResult<()> {
    let client = ClientBuilder::new_from_connection_string("grpc://localhost:2136?database=local")?
        .client()?;

    if let Ok(res) = timeout(Duration::from_secs(3), client.wait()).await {
        res?
    } else {
        return Err(YdbError::from("Connection timeout"));
    };

    let table_client = client.table_client();
    /// Create `series` table
    let prefix = "series";

    table_client.retry_execute_scheme_query(format!(
        "CREATE TABLE IF NOT EXISTS {prefix} (
            series_id Bytes,
            date Date,
            
            PRIMARY KEY(series_id));"
    ))
        .await?;

    let series_id = Uuid::new_v4().to_string();
    let rows: Vec<Value> = vec![ydb_struct!(
        "series_id" => Value::Bytes(Bytes::from(series_id.clone())),
        "date" => Value::Date(date("2008-11-21")),
    )];

    /// Fill `series` table
    let prefix = "series";

    table_client
        .retry_execute_bulk_upsert(format!("/local/{prefix}").to_string(), rows)
        .await?;

    Ok(())
}

fn date(date_str: &str) -> SystemTime {
    const DATE_ISO8601: &str = "%Y-%m-%d";
    let datetime = chrono::NaiveDate::parse_from_str(date_str, DATE_ISO8601)
        .unwrap_or_else(|_| panic!("Invalid date format: {}", date_str))
        .and_hms_opt(0, 0, 0)
        .unwrap();

    SystemTime::UNIX_EPOCH + Duration::from_secs(datetime.timestamp() as u64)
}