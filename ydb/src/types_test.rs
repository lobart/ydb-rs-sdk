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
#[ignore] // needs YDB access
async fn test_uuid() -> YdbResult<()> {
    let client = test_client_builder().client()?;

    client.wait().await?;

    let test_uuid_v7 = uuid::Uuid::now_v7();
    let test_uuid_v4 = uuid::Uuid::new_v4();

    let (v7_db_value, v4_db_value): (Option<Uuid>, Option<Uuid>) = client
        .table_client()
        .retry_transaction(|mut t| async move {
            let res = t
                .query(
                    Query::new(
                        "
                select $test_uuid_v7 as v7_db_value, $test_uuid_v4 as v4_db_value",
                    )
                    .with_params(ydb_params! {
                        "$test_uuid_v7" => test_uuid_v7,
                        "$test_uuid_v4" => test_uuid_v4,
                    }),
                )
                .await?;
            let mut row = res.into_only_row()?;
            let v7_value: Option<Uuid> = row.remove_field_by_name("v7_db_value")?.try_into()?;
            let v4_value: Option<Uuid> = row.remove_field_by_name("v4_db_value")?.try_into()?;
            Ok((v7_value, v4_value))
        })
        .await?;

    assert_eq!(test_uuid_v7, v7_db_value.unwrap());
    assert_eq!(test_uuid_v4, v4_db_value.unwrap());

    Ok(())
}
