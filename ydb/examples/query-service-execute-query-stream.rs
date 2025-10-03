use std::time::Duration;
use tokio::time::timeout;
use ydb::{ydb_struct, ClientBuilder, Query, Value, YdbError, YdbResult};
use ydb_grpc::ydb_proto::query::ExecuteQueryRequest;

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
    let table_name = "test";

    let query = Query::new(format!("SELECT * FROM {table_name} ORDER BY id"));

    query_client
        .execute_query("SELECT * FROM {table_name} ORDER BY id")
        .await?;

    println!("OK");

    Ok(())
}
