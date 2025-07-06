use std::time::Duration;
use std::{env, str::FromStr};
use tokio::time::timeout;
use tracing::{info, Level};
use ydb::{ClientBuilder, Query, ServiceAccountCredentials, YdbError, YdbResult};

#[tokio::main]
async fn main() -> YdbResult<()> {
    init_logs();
    info!("Building client");

    let connection_string =
        env::var("YDB_CONNECTION_STRING").map_err(|_| "YDB_CONNECTION_STRING not set")?;

    let client = ClientBuilder::new_from_connection_string(connection_string)?
        // get credentials from file located at path specified in YDB_SERVICE_ACCOUNT_KEY_FILE_CREDENTIALS
        .with_credentials(ServiceAccountCredentials::from_env()?)
        //  or with credentials from env:
        // .with_credentials(FromEnvCredentials::new()?)
        // or you can use custom url
        // .with_credentials(ServiceAccountCredentials::from_env()?.with_url("https://iam.api.cloud.yandex.net/iam/v1/tokens"))
        .client()?;

    info!("Waiting for client");

    if let Ok(res) = timeout(Duration::from_secs(3), client.wait()).await {
        res?
    } else {
        return Err(YdbError::from("Connection timeout"));
    };

    let sum: i32 = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t.query(Query::from("SELECT 1 + 1 as sum")).await?;
            Ok(res.into_only_row()?.remove_field_by_name("sum")?)
        })
        .await?
        .try_into()
        .unwrap();
    info!("sum: {}", sum);
    Ok(())
}

fn init_logs() {
    let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
    let log_level = Level::from_str(&level).unwrap();
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
}
