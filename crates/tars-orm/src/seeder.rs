use async_trait::async_trait;

/// A Laravel-style seeder. Implement `run` to populate tables. Seeders are
/// invoked via the CLI `tars db:seed` command or programmatically from
/// `DatabaseSeeder::run`.
#[async_trait]
pub trait Seeder: Send + Sync {
    fn name(&self) -> &'static str;

    async fn run(&self) -> Result<(), sqlx::Error>;
}
