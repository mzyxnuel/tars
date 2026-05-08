use std::path::PathBuf;

use crate::config::Config;
use crate::error::Result;
use crate::route::Router;
use crate::server::Server;

/// The top-level Application — similar to Laravel's `$app`. Owns the router,
/// config directory, and base paths. Constructed via `Application::new` and
/// booted with `.serve()`.
pub struct Application {
    pub base_path: PathBuf,
    pub router: Router,
    pub env: String,
    /// Optional directory served as a static SPA fallback. When set, any
    /// request that doesn't match a registered route falls through to
    /// `tower_http::services::ServeDir` rooted at this path, with
    /// `index.html` returned for unknown paths so client-side routers
    /// (Dioxus / TanStack / Vue Router) handle them.
    pub public_dir: Option<PathBuf>,
}

impl Application {
    pub fn new() -> Self {
        let base_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let env = std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());
        Self {
            base_path,
            router: Router::new(),
            env,
            public_dir: None,
        }
    }

    pub fn with_base_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.base_path = path.into();
        self
    }

    /// Mount a directory of static frontend assets (HTML/JS/CSS/WASM) as
    /// the SPA fallback. Resolved relative to `base_path`. Typically points
    /// at the Dioxus `dx build` output directory.
    pub fn with_public_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.public_dir = Some(path.into());
        self
    }

    /// Boot the framework: load env, parse config directory, set up tracing.
    pub fn boot(&self) -> Result<()> {
        let _ = dotenvy::dotenv();
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
        let config_dir = self.base_path.join("config");
        if let Err(e) = Config::load_dir(&config_dir) {
            tracing::warn!("Could not load config: {e}");
        }
        Ok(())
    }

    /// Helper to register routes on the app's router inline.
    pub fn routes<F: FnOnce(&mut Router)>(mut self, f: F) -> Self {
        f(&mut self.router);
        self
    }

    /// Start the HTTP server on the given address.
    pub async fn serve(self, addr: &str) -> Result<()> {
        self.boot()?;
        let public = self.public_dir.map(|p| {
            if p.is_absolute() { p } else { self.base_path.join(p) }
        });
        Server::new(addr, self.router)
            .with_public_dir(public)
            .run()
            .await
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
