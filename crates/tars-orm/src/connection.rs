use once_cell::sync::OnceCell;
use sqlx::any::{Any, AnyPoolOptions};
use sqlx::Pool;

pub type AnyPool = Pool<Any>;

static POOL: OnceCell<AnyPool> = OnceCell::new();

/// Global database handle — booted with `DB::connect` and reused for the
/// lifetime of the process. Mirrors Laravel's default DB::connection().
pub struct DB;

impl DB {
    /// Connect once using the supplied DATABASE_URL-style string. Supported
    /// prefixes: `sqlite://`, `postgres://`, `mysql://`.
    pub async fn connect(url: &str) -> Result<(), sqlx::Error> {
        sqlx::any::install_default_drivers();
        let pool = AnyPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await?;
        POOL.set(pool).map_err(|_| sqlx::Error::Configuration("DB already connected".into()))?;
        Ok(())
    }

    /// Get the global pool. Panics if `connect` hasn't been called yet —
    /// that's intentional, the same way Laravel fatals on a missing DB.
    pub fn pool() -> &'static AnyPool {
        POOL.get().expect("DB::connect was not called")
    }

    /// Execute a raw statement and return the affected row count.
    pub async fn statement(sql: &str) -> Result<u64, sqlx::Error> {
        let res: sqlx::any::AnyQueryResult = sqlx::query::<Any>(sql).execute(Self::pool()).await?;
        Ok(res.rows_affected())
    }
}

pub use once_cell;
