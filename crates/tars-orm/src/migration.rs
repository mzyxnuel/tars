use async_trait::async_trait;

use crate::connection::DB;

/// A Laravel-style migration. Implement `up` and `down` and register the
/// migration with the `MigrationRunner`.
#[async_trait]
pub trait Migration: Send + Sync {
    fn name(&self) -> &'static str;

    async fn up(&self) -> Result<(), sqlx::Error>;

    async fn down(&self) -> Result<(), sqlx::Error>;
}

/// Schema builder — a subset of Laravel's fluent schema API. Call `.table()`,
/// add columns, then `.execute()` to produce the CREATE TABLE statement.
pub struct Schema;

impl Schema {
    pub fn create(name: &str) -> TableBuilder {
        TableBuilder::new(name)
    }

    pub async fn drop(name: &str) -> Result<(), sqlx::Error> {
        let sql = format!("DROP TABLE IF EXISTS {}", name);
        sqlx::query::<sqlx::Any>(&sql).execute(DB::pool()).await?;
        Ok(())
    }
}

/// Fluent builder for a CREATE TABLE statement. Produces ANSI SQL that works
/// on SQLite, Postgres and MySQL.
pub struct TableBuilder {
    name: String,
    columns: Vec<String>,
}

impl TableBuilder {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), columns: vec![] }
    }

    pub fn id(mut self) -> Self {
        self.columns.push("id INTEGER PRIMARY KEY AUTOINCREMENT".into());
        self
    }

    pub fn string(mut self, col: &str) -> Self {
        self.columns.push(format!("{} TEXT NOT NULL", col));
        self
    }

    pub fn nullable_string(mut self, col: &str) -> Self {
        self.columns.push(format!("{} TEXT", col));
        self
    }

    pub fn integer(mut self, col: &str) -> Self {
        self.columns.push(format!("{} INTEGER NOT NULL", col));
        self
    }

    pub fn nullable_integer(mut self, col: &str) -> Self {
        self.columns.push(format!("{} INTEGER", col));
        self
    }

    pub fn boolean(mut self, col: &str) -> Self {
        self.columns.push(format!("{} BOOLEAN NOT NULL DEFAULT 0", col));
        self
    }

    pub fn float(mut self, col: &str) -> Self {
        self.columns.push(format!("{} REAL NOT NULL", col));
        self
    }

    pub fn text(mut self, col: &str) -> Self {
        self.columns.push(format!("{} TEXT", col));
        self
    }

    pub fn timestamps(mut self) -> Self {
        self.columns.push("created_at TEXT".into());
        self.columns.push("updated_at TEXT".into());
        self
    }

    pub fn raw(mut self, col: &str) -> Self {
        self.columns.push(col.to_string());
        self
    }

    /// Execute the statement against the default connection.
    pub async fn execute(self) -> Result<(), sqlx::Error> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            self.name,
            self.columns.join(", ")
        );
        sqlx::query::<sqlx::Any>(&sql).execute(DB::pool()).await?;
        Ok(())
    }
}

/// Tracks and runs pending migrations. The `migrations` table is auto-created.
pub struct MigrationRunner {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationRunner {
    pub fn new() -> Self {
        Self { migrations: vec![] }
    }

    pub fn register(mut self, m: Box<dyn Migration>) -> Self {
        self.migrations.push(m);
        self
    }

    pub async fn run(&self) -> Result<(), sqlx::Error> {
        sqlx::query::<sqlx::Any>(
            "CREATE TABLE IF NOT EXISTS migrations (name TEXT PRIMARY KEY, batch INTEGER NOT NULL)",
        )
        .execute(DB::pool())
        .await?;

        let rows: Vec<(String,)> = sqlx::query_as::<sqlx::Any, (String,)>("SELECT name FROM migrations")
            .fetch_all(DB::pool())
            .await?;
        let applied: std::collections::HashSet<String> = rows.into_iter().map(|(n,)| n).collect();

        let batch = 1_i64;
        for m in &self.migrations {
            if applied.contains(m.name()) {
                tracing::info!("Skipping {} (already applied)", m.name());
                continue;
            }
            tracing::info!("Migrating {}", m.name());
            m.up().await?;
            sqlx::query::<sqlx::Any>("INSERT INTO migrations (name, batch) VALUES ($1, $2)")
                .bind(m.name().to_string())
                .bind(batch)
                .execute(DB::pool())
                .await?;
        }
        Ok(())
    }

    pub async fn rollback(&self) -> Result<(), sqlx::Error> {
        for m in self.migrations.iter().rev() {
            tracing::info!("Rolling back {}", m.name());
            m.down().await?;
            sqlx::query::<sqlx::Any>("DELETE FROM migrations WHERE name = $1")
                .bind(m.name().to_string())
                .execute(DB::pool())
                .await?;
        }
        Ok(())
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}
