//! Boot entry point for the HTTP server. Connects the database, runs
//! pending migrations, registers routes, applies global CORS, and serves.

use std::sync::Arc;

use example_app::database::migrations::CreateUsersTable;
use example_app::routes;
use tars_core::{Application, Cors};
use tars_orm::{MigrationRunner, DB};

#[tokio::main]
async fn main() -> tars_core::Result<()> {
    let _ = dotenvy::dotenv();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://storage/app/database.sqlite?mode=rwc".into());
    DB::connect(&url)
        .await
        .map_err(|e| tars_core::Error::Internal(format!("DB connect failed: {e}")))?;
    MigrationRunner::new()
        .register(Box::new(CreateUsersTable))
        .run()
        .await
        .map_err(|e| tars_core::Error::Internal(format!("Migration failed: {e}")))?;

    let mut app = Application::new();
    routes::web::register(&mut app.router);
    routes::api::register(&mut app.router);
    app.router.apply_global(Arc::new(Cors::permissive()));

    app.serve("0.0.0.0:8000").await
}
