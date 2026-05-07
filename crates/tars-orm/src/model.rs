use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::connection::DB;
use crate::query::QueryBuilder;

/// Eloquent-like `Model` trait — implement it on a struct to get
/// `all`, `find`, `create`, `update`, `delete` helpers for free. For MVP
/// purposes we expose a query builder; derive macros can fill in the gaps.
#[async_trait]
pub trait Model: Serialize + DeserializeOwned + Send + Sync + Sized + 'static {
    /// Name of the database table — e.g. `"users"`.
    fn table() -> &'static str;

    /// Name of the primary key column — defaults to `"id"`.
    fn primary_key() -> &'static str {
        "id"
    }

    /// Convenience helper — returns a new `QueryBuilder` scoped to this model.
    fn query() -> QueryBuilder<Self> {
        QueryBuilder::new(Self::table())
    }

    /// Fetch every row of this model.
    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Self::query().get().await
    }

    /// Fetch the first row matching `primary_key = id`. The id is coerced
    /// to a number when it parses cleanly so integer-column lookups work.
    async fn find<V: ToString + Send>(id: V) -> Result<Option<Self>, sqlx::Error> {
        Self::query()
            .where_eq(Self::primary_key(), coerce_id(&id.to_string()))
            .first()
            .await
    }

    /// Insert a row constructed from the given JSON map.
    async fn create(values: serde_json::Value) -> Result<u64, sqlx::Error> {
        let map = match values {
            serde_json::Value::Object(m) => m,
            _ => return Err(sqlx::Error::Configuration("create() expects an object".into())),
        };
        let cols: Vec<String> = map.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("${}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::table(),
            cols.join(", "),
            placeholders.join(", ")
        );

        let mut q = sqlx::query::<sqlx::Any>(&sql);
        for col in &cols {
            q = bind_value(q, map.get(col).cloned().unwrap_or(serde_json::Value::Null));
        }
        let res: sqlx::any::AnyQueryResult = q.execute(DB::pool()).await?;
        Ok(res.rows_affected())
    }

    /// Delete by primary key. Returns affected rows.
    async fn delete<V: ToString + Send>(id: V) -> Result<u64, sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE {} = $1", Self::table(), Self::primary_key());
        let q = sqlx::query::<sqlx::Any>(&sql);
        let q = bind_value(q, coerce_id(&id.to_string()));
        let res: sqlx::any::AnyQueryResult = q.execute(DB::pool()).await?;
        Ok(res.rows_affected())
    }
}

/// Coerce a route-param-like id string into the right JSON type for a
/// database bind. Integer-looking ids become numbers (so they match
/// integer primary keys); everything else stays a string.
pub fn coerce_id(id: &str) -> serde_json::Value {
    if let Ok(n) = id.parse::<i64>() {
        return serde_json::Value::from(n);
    }
    serde_json::Value::String(id.to_string())
}

/// Bind a JSON value onto a sqlx query. Keeps the model helpers database
/// agnostic at the cost of stringifying non-primitive values.
pub fn bind_value<'q>(
    q: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>,
    v: serde_json::Value,
) -> sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>> {
    match v {
        serde_json::Value::Null => q.bind(None::<String>),
        serde_json::Value::Bool(b) => q.bind(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                q.bind(i)
            } else if let Some(f) = n.as_f64() {
                q.bind(f)
            } else {
                q.bind(n.to_string())
            }
        }
        serde_json::Value::String(s) => q.bind(s),
        other => q.bind(other.to_string()),
    }
}
