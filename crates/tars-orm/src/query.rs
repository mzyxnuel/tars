use serde::de::DeserializeOwned;
use sqlx::{Column, Row, TypeInfo};
use std::marker::PhantomData;

use crate::connection::DB;

/// A minimal Eloquent-style query builder — enough for MVP needs (select,
/// where, order, limit, first/get). Everything rewrites into parameterised
/// sqlx queries against the `Any` driver so SQLite/Postgres/MySQL all work.
#[derive(Debug, Clone)]
pub struct QueryBuilder<T> {
    table: String,
    wheres: Vec<(String, String, serde_json::Value)>,
    order: Option<(String, bool)>,
    limit: Option<u64>,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> QueryBuilder<T> {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            wheres: vec![],
            order: None,
            limit: None,
            _phantom: PhantomData,
        }
    }

    pub fn where_eq<V: Into<serde_json::Value>>(mut self, col: &str, val: V) -> Self {
        self.wheres.push((col.to_string(), "=".to_string(), val.into()));
        self
    }

    pub fn where_op<V: Into<serde_json::Value>>(mut self, col: &str, op: &str, val: V) -> Self {
        self.wheres.push((col.to_string(), op.to_string(), val.into()));
        self
    }

    pub fn order_by(mut self, col: &str, asc: bool) -> Self {
        self.order = Some((col.to_string(), asc));
        self
    }

    pub fn limit(mut self, n: u64) -> Self {
        self.limit = Some(n);
        self
    }

    fn build_sql(&self) -> (String, Vec<serde_json::Value>) {
        let mut sql = format!("SELECT * FROM {}", self.table);
        let mut binds = vec![];
        if !self.wheres.is_empty() {
            let parts: Vec<String> = self
                .wheres
                .iter()
                .enumerate()
                .map(|(i, (c, op, v))| {
                    binds.push(v.clone());
                    format!("{} {} ${}", c, op, i + 1)
                })
                .collect();
            sql.push_str(" WHERE ");
            sql.push_str(&parts.join(" AND "));
        }
        if let Some((col, asc)) = &self.order {
            sql.push_str(&format!(" ORDER BY {} {}", col, if *asc { "ASC" } else { "DESC" }));
        }
        if let Some(lim) = self.limit {
            sql.push_str(&format!(" LIMIT {}", lim));
        }
        (sql, binds)
    }

    /// Run the query and deserialize every row into `T` via serde_json.
    pub async fn get(self) -> Result<Vec<T>, sqlx::Error> {
        let (sql, binds) = self.build_sql();
        let mut q = sqlx::query::<sqlx::Any>(&sql);
        for b in binds {
            q = crate::model::bind_value(q, b);
        }
        let rows: Vec<sqlx::any::AnyRow> = q.fetch_all(DB::pool()).await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let v = row_to_json(&row);
            let parsed: T = serde_json::from_value(v)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            out.push(parsed);
        }
        Ok(out)
    }

    /// Return the first result or `None`.
    pub async fn first(self) -> Result<Option<T>, sqlx::Error> {
        let mut rows = self.limit(1).get().await?;
        Ok(rows.pop())
    }
}

/// Convert an arbitrary sqlx `AnyRow` into a `serde_json::Value` object.
/// Used so the query builder can stay generic over the driver.
pub fn row_to_json(row: &sqlx::any::AnyRow) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for col in row.columns() {
        let name = col.name().to_string();
        let type_name = col.type_info().name();
        let value = match type_name {
            "INTEGER" | "BIGINT" | "INT8" | "INT4" | "INT2" => {
                row.try_get::<i64, _>(col.ordinal())
                    .map(|v| serde_json::Value::from(v))
                    .unwrap_or(serde_json::Value::Null)
            }
            "REAL" | "FLOAT" | "DOUBLE" | "FLOAT8" | "NUMERIC" => row
                .try_get::<f64, _>(col.ordinal())
                .map(|v| serde_json::json!(v))
                .unwrap_or(serde_json::Value::Null),
            "BOOLEAN" | "BOOL" => row
                .try_get::<bool, _>(col.ordinal())
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
            _ => row
                .try_get::<String, _>(col.ordinal())
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null),
        };
        obj.insert(name, value);
    }
    serde_json::Value::Object(obj)
}
