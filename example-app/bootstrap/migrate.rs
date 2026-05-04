//! Run all registered migrations. Invoked via `cargo run --bin migrate`
//! or through the CLI `tars migrate`.

use example_app::database::migrations::CreateUsersTable;
use tars_orm::{MigrationRunner, DB};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".into());
    DB::connect(&url).await?;
    let runner = MigrationRunner::new().register(Box::new(CreateUsersTable));
    runner.run().await?;
    println!("Migrations complete.");
    Ok(())
}
