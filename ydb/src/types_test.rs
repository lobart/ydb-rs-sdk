use crate::client::Client;
use crate::{test_helpers::test_client_builder, ydb_params, Query, Value, YdbResult};
use uuid::Uuid;

#[test]
fn test_is_optional() -> YdbResult<()> {
    assert!(Value::optional_from(Value::Bool(false), None)?.is_optional());
    assert!(Value::optional_from(Value::Bool(false), Some(Value::Bool(false)))?.is_optional());
    assert!(!Value::Bool(false).is_optional());
    Ok(())
}

#[tokio::test]
#[ignore] // need YDB access
async fn test_decimal() -> YdbResult<()> {
    let client = test_client_builder().client()?;

    client.wait().await?;

    let db_value: Option<decimal_rs::Decimal> = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t
                .query(Query::from(
                    "select CAST(\"-1233333333333333333333345.34\" AS Decimal(28, 2)) as db_value",
                ))
                .await?;
            Ok(res.into_only_row()?.remove_field_by_name("db_value")?)
        })
        .await?
        .try_into()
        .unwrap();
    let test_value = Some(
        "-1233333333333333333333345.34"
            .parse::<decimal_rs::Decimal>()
            .unwrap(),
    );
    assert_eq!(test_value, db_value);

    Ok(())
}

#[tokio::test]
#[ignore = "needs YDB access"]
async fn test_uuid_serialization() -> YdbResult<()> {
    let client = test_client_builder().client()?;
    client.wait().await?;

    let test_cases: Vec<Uuid> = vec![
        (uuid::Uuid::now_v7()),
        (uuid::Uuid::new_v4()),
        (uuid::Uuid::nil()),
        (Uuid::from_u128(0x1234567890abcdef1234567890abcdef)),
    ];

    for test_uuid in &test_cases {
        check_uuid_as_uuid_serialization(&client, *test_uuid).await?;
    }

    for test_uuid in &test_cases {
        check_uuid_as_utf8_serialization(&client, *test_uuid).await?;
    }

    for test_uuid in &test_cases {
        check_text_as_uuid_serialization(&client, *test_uuid).await?;
    }

    Ok(())
}

async fn check_uuid_as_uuid_serialization(client: &Client, test_uuid: Uuid) -> YdbResult<()> {
    let (db_value,): (Option<Uuid>,) = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t
                .query(
                    Query::new("select $test_uuid as db_value").with_params(ydb_params! {
                        "$test_uuid" => test_uuid,
                    }),
                )
                .await?;
            let mut row = res.into_only_row()?;
            let value: Option<Uuid> = row.remove_field_by_name("db_value")?.try_into()?;
            Ok((value,))
        })
        .await?;

    assert_eq!(Some(test_uuid), db_value);
    Ok(())
}

async fn check_uuid_as_utf8_serialization(client: &Client, test_uuid: Uuid) -> YdbResult<()> {
    let (db_result,): (Option<String>,) = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t
                .query(
                    Query::new(
                        "
                declare $test_uuid AS Uuid;
                select cast($test_uuid AS Utf8) AS db_result",
                    )
                    .with_params(ydb_params! {
                        "$test_uuid" => test_uuid,
                    }),
                )
                .await?;
            let mut row = res.into_only_row()?;

            let value: Option<String> = row.remove_field_by_name("db_result")?.try_into()?;
            Ok((value,))
        })
        .await?;

    assert_eq!(Some(test_uuid.to_string()), db_result);
    Ok(())
}

async fn check_text_as_uuid_serialization(client: &Client, test_uuid: Uuid) -> YdbResult<()> {
    let (db_result,): (Option<Uuid>,) = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t
                .query(
                    Query::new(
                        "
            declare $val AS Text;
            select cast($val AS UUID) AS db_result",
                    )
                    .with_params(ydb_params! {
                        "$val" => test_uuid.to_string(),
                    }),
                )
                .await?;
            let mut row = res.into_only_row()?;

            let value: Option<Uuid> = row.remove_field_by_name("db_result")?.try_into()?;
            Ok((value,))
        })
        .await?;

    assert_eq!(Some(test_uuid), db_result);
    Ok(())
}
