use clap::{Parser, Subcommand};

mod generator;
mod scaffold;

/// TARS CLI — artisan-like codegen and project tooling.
#[derive(Parser)]
#[command(name = "tars", about = "TARS framework CLI (artisan-like)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new TARS project scaffold in the given directory.
    New {
        name: String,
    },
    /// Generators — `tars make:<kind> <Name>`.
    #[command(name = "make:controller")]
    MakeController { name: String },
    #[command(name = "make:model")]
    MakeModel { name: String },
    #[command(name = "make:migration")]
    MakeMigration { name: String },
    #[command(name = "make:seeder")]
    MakeSeeder { name: String },
    #[command(name = "make:factory")]
    MakeFactory { name: String },
    #[command(name = "make:request")]
    MakeRequest { name: String },
    #[command(name = "make:resource")]
    MakeResource { name: String },
    /// Start the development server (delegates to `cargo run --bin server`).
    Serve,
    /// Run registered migrations (delegates to `cargo run --bin migrate`).
    Migrate,
    /// Run seeders (delegates to `cargo run --bin seed`).
    #[command(name = "db:seed")]
    Seed,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).init();
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => scaffold::new_project(&name),
        Commands::MakeController { name } => generator::make_controller(&name),
        Commands::MakeModel { name } => generator::make_model(&name),
        Commands::MakeMigration { name } => generator::make_migration(&name),
        Commands::MakeSeeder { name } => generator::make_seeder(&name),
        Commands::MakeFactory { name } => generator::make_factory(&name),
        Commands::MakeRequest { name } => generator::make_request(&name),
        Commands::MakeResource { name } => generator::make_resource(&name),
        Commands::Serve => {
            let status = std::process::Command::new("cargo")
                .args(["run", "--bin", "server"])
                .status()?;
            std::process::exit(status.code().unwrap_or(0));
        }
        Commands::Migrate => {
            let status = std::process::Command::new("cargo")
                .args(["run", "--bin", "migrate"])
                .status()?;
            std::process::exit(status.code().unwrap_or(0));
        }
        Commands::Seed => {
            let status = std::process::Command::new("cargo")
                .args(["run", "--bin", "seed"])
                .status()?;
            std::process::exit(status.code().unwrap_or(0));
        }
    }
}
