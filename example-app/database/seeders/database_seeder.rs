use async_trait::async_trait;
use tars_orm::Seeder;

use super::UserSeeder;

/// Top-level seeder — mirrors Laravel's `DatabaseSeeder`. Kicks off every
/// sub-seeder in order.
pub struct DatabaseSeeder;

#[async_trait]
impl Seeder for DatabaseSeeder {
    fn name(&self) -> &'static str {
        "DatabaseSeeder"
    }

    async fn run(&self) -> Result<(), sqlx::Error> {
        UserSeeder.run().await?;
        Ok(())
    }
}
