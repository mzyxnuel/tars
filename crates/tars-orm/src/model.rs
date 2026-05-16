use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use crate::connection::DB;
use crate::query::QueryBuilder;

/// Eloquent-like `Model` trait — implement it on a struct to get `all`,
/// `find`, `create`, `update_by_id`, `delete_by_id` for free. Each
/// `create`/`update_*` returns the persisted `Self` (not a row count) so
/// callers can return the model straight to a `JsonResource`.
#[async_trait]
pub trait Model: Serialize + DeserializeOwned + Send + Sync + Sized + 'static {
    /// Name of the database table — e.g. `"users"`.
    fn table() -> &'static str;

    /// Name of the primary key column — defaults to `"id"`.
    fn primary_key() -> &'static str {
        "id"
    }

    /// When `true` (the default), `create` and `update_by_id` auto-fill
    /// `created_at` / `updated_at` columns with the current UTC time
    /// (RFC 3339). Override to opt out for tables without those columns.
    fn uses_timestamps() -> bool {
        true
    }

    /// Convenience helper — returns a new `QueryBuilder` scoped to this model.
    fn query() -> QueryBuilder<Self> {
        QueryBuilder::new(Self::table())
    }

    /// Fetch every row.
    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Self::query().get().await
    }

    /// Fetch the first row matching `primary_key = id`. Numeric ids are
    /// coerced to integers so integer-column lookups work via the `Any`
    /// driver.
    async fn find<V: ToString + Send>(id: V) -> Result<Option<Self>, sqlx::Error> {
        Self::query()
            .where_eq(Self::primary_key(), coerce_id(&id.to_string()))
            .first()
            .await
    }

    /// Insert a row from the given JSON object and return the freshly
    /// persisted model. `created_at` / `updated_at` are filled
    /// automatically when [`uses_timestamps`](Self::uses_timestamps) is
    /// true. Uses `INSERT ... RETURNING *` so the row is returned in a
    /// single round-trip (requires SQLite 3.35+ or Postgres; MySQL 8 has
    /// no RETURNING).
    async fn create(values: Value) -> Result<Self, sqlx::Error> {
        let mut map = match values {
            Value::Object(m) => m,
            _ => return Err(sqlx::Error::Configuration("create() expects an object".into())),
        };
        if Self::uses_timestamps() {
            let now = current_timestamp();
            map.entry("created_at".to_string()).or_insert_with(|| Value::String(now.clone()));
            map.entry("updated_at".to_string()).or_insert(Value::String(now));
        }

        let cols: Vec<String> = map.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=cols.len()).map(|i| format!("${}", i)).collect();
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            Self::table(),
            cols.join(", "),
            placeholders.join(", ")
        );
        let mut q = sqlx::query::<sqlx::Any>(&sql);
        for col in &cols {
            q = bind_value(q, map.get(col).cloned().unwrap_or(Value::Null));
        }
        let row: sqlx::any::AnyRow = q.fetch_one(DB::pool()).await?;
        let json = crate::query::row_to_json(&row);
        serde_json::from_value(json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    /// Update by primary key and return the refreshed model. `updated_at`
    /// is bumped automatically when [`uses_timestamps`](Self::uses_timestamps) is true.
    async fn update_by_id<V: ToString + Send>(
        id: V,
        values: Value,
    ) -> Result<Self, sqlx::Error> {
        let mut map: Map<String, Value> = match values {
            Value::Object(m) => m,
            _ => return Err(sqlx::Error::Configuration("update_by_id expects an object".into())),
        };
        if Self::uses_timestamps() {
            map.insert("updated_at".to_string(), Value::String(current_timestamp()));
        }
        if map.is_empty() {
            // Nothing to update — return the current row as-is.
            return Self::find(id).await?.ok_or(sqlx::Error::RowNotFound);
        }

        let id_str = id.to_string();
        let cols: Vec<String> = map.keys().cloned().collect();
        let sets: Vec<String> = cols
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{} = ${}", c, i + 1))
            .collect();
        let id_idx = cols.len() + 1;
        let sql = format!(
            "UPDATE {} SET {} WHERE {} = ${} RETURNING *",
            Self::table(),
            sets.join(", "),
            Self::primary_key(),
            id_idx
        );
        let mut q = sqlx::query::<sqlx::Any>(&sql);
        for col in &cols {
            q = bind_value(q, map.get(col).cloned().unwrap_or(Value::Null));
        }
        q = bind_value(q, coerce_id(&id_str));
        let row: sqlx::any::AnyRow = q.fetch_one(DB::pool()).await?;
        let json = crate::query::row_to_json(&row);
        serde_json::from_value(json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
    }

    /// Delete by primary key. Returns the number of affected rows.
    async fn delete_by_id<V: ToString + Send>(id: V) -> Result<u64, sqlx::Error> {
        let sql = format!("DELETE FROM {} WHERE {} = $1", Self::table(), Self::primary_key());
        let q = sqlx::query::<sqlx::Any>(&sql);
        let q = bind_value(q, coerce_id(&id.to_string()));
        let res: sqlx::any::AnyQueryResult = q.execute(DB::pool()).await?;
        Ok(res.rows_affected())
    }
}

/// Instance-side helpers — automatically implemented for every `Model`.
/// Lets controllers write `user.delete().await?` or `user.update(payload)`.
#[async_trait]
pub trait ModelInstance: Model {
    /// Delete this instance.
    async fn delete(&self) -> Result<u64, sqlx::Error> {
        let v = serde_json::to_value(self)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let id = v
            .get(Self::primary_key())
            .cloned()
            .ok_or_else(|| sqlx::Error::ColumnNotFound(Self::primary_key().to_string()))?;
        Self::delete_by_id(value_as_string(&id)).await
    }

    /// Update this instance and return the refreshed model.
    async fn update(self, values: Value) -> Result<Self, sqlx::Error> {
        let v = serde_json::to_value(&self)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let id = v
            .get(Self::primary_key())
            .cloned()
            .ok_or_else(|| sqlx::Error::ColumnNotFound(Self::primary_key().to_string()))?;
        Self::update_by_id(value_as_string(&id), values).await
    }
}

impl<T: Model> ModelInstance for T {}

/// Wire route-model binding for a `Model` type. Expands to a `RouteBindable`
/// impl that delegates to `Model::find`, so `Controller::Model = User`
/// just works. Invoke once per model, next to its `Model` impl:
///
/// ```ignore
/// impl Model for User { fn table() -> &'static str { "users" } }
/// tars_orm::bind_model!(User);
/// ```
///
/// (Implemented as `macro_rules!` rather than a blanket impl so the
/// `RouteBindable` impl is generated in the user's crate where Rust's
/// orphan rule accepts it.)
#[macro_export]
macro_rules! bind_model {
    ($t:ty) => {
        #[::async_trait::async_trait]
        impl $crate::__macro_support::tars_core::binding::RouteBindable for $t {
            async fn route_bind(
                id: &str,
            ) -> $crate::__macro_support::tars_core::error::Result<Option<Self>> {
                match <$t as $crate::Model>::find(id).await {
                    Ok(opt) => Ok(opt),
                    Err(e) => Err(
                        $crate::__macro_support::tars_core::error::Error::Internal(e.to_string()),
                    ),
                }
            }
        }
    };
}


/// Coerce a route-param-like id string into the right JSON type for a
/// database bind. Integer-looking ids become numbers (so they match
/// integer primary keys); everything else stays a string.
pub fn coerce_id(id: &str) -> Value {
    if let Ok(n) = id.parse::<i64>() {
        return Value::from(n);
    }
    Value::String(id.to_string())
}

fn value_as_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Bind a JSON value onto a sqlx query. Keeps the model helpers
/// database-agnostic at the cost of stringifying non-primitive values.
pub fn bind_value<'q>(
    q: sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>>,
    v: Value,
) -> sqlx::query::Query<'q, sqlx::Any, sqlx::any::AnyArguments<'q>> {
    match v {
        Value::Null => q.bind(None::<String>),
        Value::Bool(b) => q.bind(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                q.bind(i)
            } else if let Some(f) = n.as_f64() {
                q.bind(f)
            } else {
                q.bind(n.to_string())
            }
        }
        Value::String(s) => q.bind(s),
        other => q.bind(other.to_string()),
    }
}
