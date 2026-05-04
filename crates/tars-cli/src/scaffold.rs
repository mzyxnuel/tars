use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Create a new TARS project with the Laravel 13 directory tree.
pub fn new_project(name: &str) -> Result<()> {
    let root = PathBuf::from(name);
    if root.exists() {
        anyhow::bail!("Directory '{name}' already exists.");
    }

    // Laravel 13 directory tree (minus /app/Models — models live at /models).
    let dirs = [
        "app/Console/Commands",
        "app/Http/Controllers",
        "app/Http/Middleware",
        "app/Http/Requests",
        "app/Http/Resources",
        "app/Providers",
        "bootstrap",
        "config",
        "database/factories",
        "database/migrations",
        "database/seeders",
        "models",
        "public",
        "resources/views",
        "resources/routes",
        "routes",
        "storage/app",
        "storage/framework",
        "storage/logs",
        "tests/Feature",
        "tests/Unit",
    ];
    for d in dirs {
        fs::create_dir_all(root.join(d))?;
    }

    fs::write(
        root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
tars-core = "0.1"
tars-orm = "0.1"
tars-validation = "0.1"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
async-trait = "0.1"

[[bin]]
name = "server"
path = "bootstrap/server.rs"

[[bin]]
name = "migrate"
path = "bootstrap/migrate.rs"

[[bin]]
name = "seed"
path = "bootstrap/seed.rs"
"#
        ),
    )?;

    fs::write(
        root.join("config/app.toml"),
        "name = \"TARS App\"\nurl = \"http://localhost:8000\"\n",
    )?;
    fs::write(
        root.join("config/database.toml"),
        "default = \"sqlite\"\n\n[sqlite]\nurl = \"sqlite://storage/app/database.sqlite\"\n",
    )?;

    fs::write(
        root.join(".env"),
        "APP_ENV=local\nAPP_DEBUG=true\nDATABASE_URL=sqlite://storage/app/database.sqlite\n",
    )?;

    fs::write(
        root.join("routes/api.rs"),
        "// Register API routes on the provided `router` — called from bootstrap/server.rs.\nuse tars_core::Router;\n\npub fn register(router: &mut Router) {\n    router.get(\"/api/health\", |_req| async move {\n        Ok(tars_core::Response::json(serde_json::json!({ \"ok\": true })))\n    });\n}\n",
    )?;

    fs::write(
        root.join("bootstrap/server.rs"),
        r#"use tars_core::Application;

#[path = "../routes/api.rs"]
mod api;

#[tokio::main]
async fn main() -> tars_core::Result<()> {
    let mut app = Application::new();
    api::register(&mut app.router);
    app.serve("0.0.0.0:8000").await
}
"#,
    )?;

    fs::write(
        root.join("bootstrap/migrate.rs"),
        r#"use tars_orm::{MigrationRunner, DB};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_URL")?;
    DB::connect(&url).await?;
    let runner = MigrationRunner::new();
    runner.run().await?;
    Ok(())
}
"#,
    )?;

    fs::write(
        root.join("bootstrap/seed.rs"),
        r#"use tars_orm::DB;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::var("DATABASE_URL")?;
    DB::connect(&url).await?;
    println!("Seeding completed.");
    Ok(())
}
"#,
    )?;

    fs::write(
        root.join("README.md"),
        format!("# {name}\n\nA TARS (Laravel-in-Rust) application.\n"),
    )?;

    println!("Created project '{name}' with Laravel 13 directory tree.");
    Ok(())
}
