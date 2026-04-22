//! Run the top-level `DatabaseSeeder`. Invoked via `cargo run --bin seed`
//! or through the CLI `tars db:seed`.

use example_app::database::seeders::DatabaseSeeder;
use tars_orm::{Seeder, DB};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".into());
    DB::connect(&url).await?;
    DatabaseSeeder.run().await?;
    println!("Seeding complete.");
    Ok(())
}
