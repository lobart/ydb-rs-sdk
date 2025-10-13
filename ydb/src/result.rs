use std::collections::HashMap;
use std::sync::Arc;
use std::vec::IntoIter;

use itertools::Itertools;
use tracing::trace;

use ydb_grpc::ydb_proto::issue;
use ydb_grpc::ydb_proto::issue::IssueMessage;
use ydb_grpc::ydb_proto::query::ExecuteQueryResponsePart;
use ydb_grpc::ydb_proto::status_ids::StatusCode;
use ydb_grpc::ydb_proto::table::ExecuteScanQueryPartialResponse;

use crate::errors;
use crate::errors::{YdbError, YdbResult, YdbStatusError};
use crate::grpc::proto_issues_to_ydb_issues;
use crate::grpc_wrapper::raw_table_service::execute_data_query::RawExecuteDataQueryResult;
use crate::grpc_wrapper::value::{RawResultSet, RawTypedValue, RawValue};
use crate::trace_helpers::ensure_len_string;
use crate::types::Value;

#[derive(Debug)]
pub struct QueryResult {
    pub(crate) results: Vec<ResultSet>,
    pub(crate) tx_id: String,
}

impl QueryResult {
    pub(crate) fn from_raw_result(
        error_on_truncate: bool,
        raw_res: RawExecuteDataQueryResult,
    ) -> YdbResult<Self> {
        trace!(
            "raw_res: {}",
            ensure_len_string(serde_json::to_string(&raw_res)?)
        );
        let mut results = Vec::with_capacity(raw_res.result_sets.len());
        for current_set in raw_res.result_sets.into_iter() {
            if error_on_truncate && current_set.truncated {
                return Err(
                    format!("got truncated result. result set index: {}", results.len())
                        .as_str()
                        .into(),
                );
            }
            let result_set = ResultSet::try_from(current_set)?;

            results.push(result_set);
        }

        Ok(QueryResult {
            results,
            tx_id: raw_res.tx_meta.id,
        })
    }

    pub fn into_only_result(self) -> YdbResult<ResultSet> {
        let mut iter = self.results.into_iter();
        match iter.next() {
            Some(result_set) => {
                if iter.next().is_none() {
                    Ok(result_set)
                } else {
                    Err(YdbError::from_str("more then one result set"))
                }
            }
            None => Err(YdbError::from_str("no result set")),
        }
    }

    pub fn into_only_row(self) -> YdbResult<Row> {
        let result_set = self.into_only_result()?;
        let mut rows = result_set.rows();
        match rows.next() {
            Some(first_row) => {
                if rows.next().is_none() {
                    Ok(first_row)
                } else {
                    Err(YdbError::from_str("result set has more then one row"))
                }
            }
            None => Err(YdbError::NoRows),
        }
    }
}

#[derive(Debug)]
pub struct ResultSet {
    columns: Vec<crate::types::Column>,
    columns_by_name: HashMap<String, usize>,
    raw_result_set: RawResultSet,
}

impl ResultSet {
    #[allow(dead_code)]
    pub(crate) fn columns(&self) -> &Vec<crate::types::Column> {
        &self.columns
    }

    pub fn rows(self) -> ResultSetRowsIter {
        ResultSetRowsIter {
            columns: Arc::new(self.columns),
            columns_by_name: Arc::new(self.columns_by_name),
            row_iter: self.raw_result_set.rows.into_iter(),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn truncated(&self) -> bool {
        self.raw_result_set.truncated
    }
}

impl TryFrom<RawResultSet> for ResultSet {
    type Error = YdbError;

    fn try_from(value: RawResultSet) -> Result<Self, Self::Error> {
        let columns_by_name: HashMap<String, usize> = value
            .columns
            .iter()
            .enumerate()
            .map(|(index, column)| (column.name.clone(), index))
            .collect();
        Ok(Self {
            columns: value
                .columns
                .iter()
                .map(|item| item.clone().try_into())
                .try_collect()?,
            columns_by_name,
            raw_result_set: value,
        })
    }
}

impl IntoIterator for ResultSet {
    type Item = Row;
    type IntoIter = ResultSetRowsIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows()
    }
}

#[derive(Debug)]
pub struct Row {
    columns: Arc<Vec<crate::types::Column>>,
    columns_by_name: Arc<HashMap<String, usize>>,
    raw_values: HashMap<usize, RawValue>,
}

impl Row {
    pub fn remove_field_by_name(&mut self, name: &str) -> errors::YdbResult<Value> {
        if let Some(&index) = self.columns_by_name.get(name) {
            return self.remove_field(index);
        }
        Err(YdbError::Custom("field not found".into()))
    }

    pub fn remove_field(&mut self, index: usize) -> errors::YdbResult<Value> {
        match self.raw_values.remove(&index) {
            Some(val) => Ok(Value::try_from(RawTypedValue {
                r#type: self.columns[index].v_type.clone(),
                value: val,
            })?),
            None => Err(YdbError::Custom("it has no the field".into())),
        }
    }
}

pub struct ResultSetRowsIter {
    columns: Arc<Vec<crate::types::Column>>,
    columns_by_name: Arc<HashMap<String, usize>>,
    row_iter: IntoIter<Vec<RawValue>>,
}

impl Iterator for ResultSetRowsIter {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        match self.row_iter.next() {
            None => None,
            Some(row) => Some(Row {
                columns: self.columns.clone(),
                columns_by_name: self.columns_by_name.clone(),
                raw_values: row.into_iter().enumerate().collect(),
            }),
        }
    }
}

//refactor with generic for table and session
pub struct StreamResult<T> {
    pub(crate) results: tonic::codec::Streaming<T>,
}

pub(crate) trait StreamResultTrait {
    fn status(&self) -> YdbResult<i32>;
    fn issues(&self) -> YdbResult<&Vec<IssueMessage>>;
    fn result_set(&self) -> YdbResult<Option<ResultSet>>;
}

impl StreamResultTrait for ExecuteQueryResponsePart {
    fn status(&self) -> YdbResult<i32> {
        Ok(self.status)
    }

    fn issues(&self) -> YdbResult<&Vec<IssueMessage>> {
        Ok(&self.issues)
    }

    fn result_set(&self) -> YdbResult<Option<ResultSet>> {
        let Some(proto_result_set) = self.result_set.clone() else {
            return Ok(None);
        };

        let raw_res = RawResultSet::try_from(proto_result_set)?;
        let result_set = ResultSet::try_from(raw_res)?;
        Ok(Some(result_set))
    }
}

impl StreamResultTrait for ExecuteScanQueryPartialResponse {
    fn status(&self) -> YdbResult<i32> {
        Ok(self.status)
    }

    fn issues(&self) -> YdbResult<&Vec<IssueMessage>> {
        Ok(&self.issues)
    }

    fn result_set(&self) -> YdbResult<Option<ResultSet>> {
        let proto_result_set = match &self.result {
            Some(result) => result.result_set.as_ref(),
            None => return Err(YdbError::InternalError("unexpected empty result".into())),
        };

        let proto_result_set = match proto_result_set {
            Some(proto) => proto,
            None => return Ok(None),
        };

        let raw_res = RawResultSet::try_from(proto_result_set.clone())?;
        let result_set = ResultSet::try_from(raw_res)?;
        Ok(Some(result_set))
    }
}

impl<T: StreamResultTrait> StreamResult<T> {
    pub async fn next(&mut self) -> YdbResult<Option<ResultSet>> {
        let partial_response = self
            .results
            .message()
            .await?
            .ok_or(YdbError::EmptyResponse)?;

        let status = partial_response.status()?;

        if status != StatusCode::Success as i32 {
            println!("status {:?}", status);
            let issues = partial_response.issues()?;
            return Err(YdbError::YdbStatusError(YdbStatusError {
                message: issues
                    .iter()
                    .map(|issue| issue.message.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
                operation_status: status,
                issues: proto_issues_to_ydb_issues(issues),
            }));
        }

        partial_response.result_set()
    }
}

pub(crate) type StreamTableResult = StreamResult<ExecuteScanQueryPartialResponse>;
pub(crate) type StreamQueryResult = StreamResult<ExecuteQueryResponsePart>;
