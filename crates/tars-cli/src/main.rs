use clap::{Parser, Subcommand};

mod generator;
mod scaffold;

/// TARS CLI — artisan-like codegen, runtime, and frontend tooling.
///
/// Wraps `cargo` for backend binaries (server, migrate, seed) and the
/// Dioxus CLI (`dx`) for the frontend, so a single `tars` invocation
/// covers every workflow in a project. Run from the project root.
#[derive(Parser)]
#[command(name = "tars", about = "TARS framework CLI", version)]
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

    // ----- Generators (`tars make:<kind> <Name>`) -----------------------
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

    // ----- Backend lifecycle (wraps cargo) -----------------------------
    /// Start the HTTP server (wraps `cargo run --bin server`).
    Serve {
        /// Build with optimisations on (`cargo run --release`).
        #[arg(long)]
        release: bool,
    },
    /// Run registered migrations (wraps `cargo run --bin migrate`).
    Migrate {
        #[arg(long)]
        release: bool,
    },
    /// Run database seeders (wraps `cargo run --bin seed`).
    #[command(name = "db:seed")]
    Seed {
        #[arg(long)]
        release: bool,
    },

    // ----- Frontend lifecycle (wraps `dx`) -----------------------------
    /// Start the frontend dev server with hot reload
    /// (wraps `dx serve --web` inside `resources/`).
    Dev {
        /// Override the dev server port (forwarded as `--port`).
        #[arg(long)]
        port: Option<u16>,
        /// Target platform passed to dx as `--web` / `--desktop` /
        /// `--mobile` / `--ios` / `--android`. Defaults to `web`.
        #[arg(long, default_value = "web")]
        platform: String,
    },
    /// Build the frontend bundle (wraps `dx build --<platform>` inside `resources/`).
    Build {
        /// Add `--release` to the dx build.
        #[arg(long)]
        release: bool,
        /// Target platform passed to dx as `--web` / `--desktop` /
        /// `--mobile` / `--ios` / `--android`. Defaults to `web`.
        #[arg(long, default_value = "web")]
        platform: String,
    },
    /// Pass arbitrary args through to `dx` inside `resources/`.
    /// Example: `tars dx -- --help`.
    Dx {
        /// Args forwarded verbatim to `dx`.
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() {
    tracing_subscriber::fmt().with_target(false).init();
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
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

        Commands::Serve { release } => run_cargo_bin("server", release),
        Commands::Migrate { release } => run_cargo_bin("migrate", release),
        Commands::Seed { release } => run_cargo_bin("seed", release),

        Commands::Dev { port, platform } => run_dx_dev(port, &platform),
        Commands::Build { release, platform } => run_dx_build(release, &platform),
        Commands::Dx { args } => run_dx(&args),
    }
}

/// Forward to `cargo run [--release] --bin <name>`. Inherits stdio so the
/// child's output (server logs, migration progress, etc.) streams live.
fn run_cargo_bin(bin: &str, release: bool) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run");
    if release {
        cmd.arg("--release");
    }
    cmd.args(["--bin", bin]);
    println!("→ cargo run{} --bin {bin}", if release { " --release" } else { "" });
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(0));
}

/// Pinned to the Dioxus version declared in the workspace `Cargo.toml`. The
/// dioxus-cli release line tracks the dioxus crate, so the two must match
/// for `dx serve` to produce a working bundle.
const DIOXUS_CLI_VERSION: &str = "0.7.9";

/// Locate the `dx` executable on PATH, falling back to the cargo install
/// bin directory in case the user just installed it and hasn't shimmed
/// PATH yet (common on fresh boxes).
fn find_dx() -> Option<std::path::PathBuf> {
    let exe = if cfg!(windows) { "dx.exe" } else { "dx" };
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let p = dir.join(exe);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    if let Some(bin) = cargo_install_bin() {
        let p = bin.join(exe);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn cargo_install_bin() -> Option<std::path::PathBuf> {
    if let Some(h) = std::env::var_os("CARGO_HOME") {
        return Some(std::path::PathBuf::from(h).join("bin"));
    }
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(std::path::PathBuf::from(home).join(".cargo").join("bin"))
}

/// Return a usable path to `dx`, installing dioxus-cli on the fly when
/// missing so `tars dev` / `tars build` work out of the box without an
/// extra setup step. The install runs once per machine.
fn ensure_dx() -> anyhow::Result<std::path::PathBuf> {
    if let Some(p) = find_dx() {
        return Ok(p);
    }
    println!(
        "→ Dioxus CLI (`dx`) not found. Installing dioxus-cli {DIOXUS_CLI_VERSION} via \
         `cargo install` — first-time setup can take a few minutes."
    );
    let status = std::process::Command::new("cargo")
        .args([
            "install",
            "dioxus-cli",
            "--version",
            DIOXUS_CLI_VERSION,
            "--locked",
        ])
        .status()
        .map_err(|e| anyhow::anyhow!("failed to spawn `cargo install`: {e}"))?;
    if !status.success() {
        anyhow::bail!(
            "`cargo install dioxus-cli --version {DIOXUS_CLI_VERSION} --locked` failed \
             (exit status: {status}). Re-run manually to see the full error."
        );
    }
    find_dx().ok_or_else(|| {
        anyhow::anyhow!(
            "dioxus-cli installed, but `dx` was still not found. Add \
             `$CARGO_HOME/bin` (or `~/.cargo/bin`) to your PATH and retry."
        )
    })
}

/// Run `dx` from the frontend crate. Defaults to `./resources/` since that's
/// where `tars new` puts the Dioxus crate. Auto-installs dioxus-cli when
/// `dx` isn't already on PATH.
fn run_dx(args: &[String]) -> anyhow::Result<()> {
    let resources = std::path::Path::new("resources");
    if !resources.exists() {
        anyhow::bail!(
            "`resources/` directory not found in current working directory. \
             Run `tars dx ...` from your project root."
        );
    }
    let dx = ensure_dx()?;
    let mut cmd = std::process::Command::new(&dx);
    cmd.current_dir(resources).args(args);
    let pretty: Vec<&str> = args.iter().map(String::as_str).collect();
    println!("→ (cd resources && dx {})", pretty.join(" "));
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(0));
}

fn run_dx_dev(port: Option<u16>, platform: &str) -> anyhow::Result<()> {
    let mut args = vec!["serve".to_string(), platform_flag(platform)?];
    if let Some(p) = port {
        args.push("--port".into());
        args.push(p.to_string());
    }
    run_dx(&args)
}

fn run_dx_build(release: bool, platform: &str) -> anyhow::Result<()> {
    let mut args = vec!["build".to_string(), platform_flag(platform)?];
    if release {
        args.push("--release".into());
    }
    run_dx(&args)
}

/// Translate the `--platform <name>` argument we expose into the bare flag
/// the Dioxus CLI 0.7 expects. Each flag sets target triple, cargo features,
/// and build profile all at once — `dx serve --features web` no longer
/// works in 0.7 (the CLI cannot infer the target triple from it).
fn platform_flag(name: &str) -> anyhow::Result<String> {
    Ok(match name {
        "web" => "--web".into(),
        "desktop" => "--desktop".into(),
        "mobile" => "--mobile".into(),
        "ios" => "--ios".into(),
        "android" => "--android".into(),
        other => anyhow::bail!(
            "unknown --platform `{other}`. Supported: web, desktop, mobile, ios, android."
        ),
    })
}
