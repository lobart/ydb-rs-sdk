use tracing::trace;
use tracing_test::traced_test;
use crate::test_integration_helper::create_client;
use crate::YdbResult;

#[tokio::test]
#[traced_test]
#[ignore] // need YDB access
async fn create_session() -> YdbResult<()> {
    let res = create_client()
        .await?;
    trace!("session: {:?}", res);
    Ok(())
}