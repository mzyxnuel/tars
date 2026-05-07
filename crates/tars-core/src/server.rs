use axum::body::Body;
use axum::extract::State;
use axum::http::Request as AxumRequest;
use axum::routing::{delete, get, options, patch, post, put};
use axum::Router as AxumRouter;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use crate::error::{Error, Result};
use crate::request::Request;
use crate::response::Response;
use crate::route::{build_stack, Method, Router};

#[derive(Clone)]
struct HandlerEntry {
    handler: crate::route::Handler,
    path_template: String,
}

#[derive(Clone)]
struct ServerState {
    routes: Arc<HashMap<(Method, String), HandlerEntry>>,
}

/// The low-level HTTP server — wraps axum and turns a `Router` into a live
/// service. Not usually constructed directly; use `Application::serve`.
pub struct Server {
    pub addr: String,
    pub router: Router,
    pub public_dir: Option<PathBuf>,
}

impl Server {
    pub fn new(addr: impl Into<String>, router: Router) -> Self {
        Self { addr: addr.into(), router, public_dir: None }
    }

    /// Set a directory of static assets served as an SPA fallback.
    pub fn with_public_dir(mut self, dir: Option<PathBuf>) -> Self {
        self.public_dir = dir;
        self
    }

    pub async fn run(self) -> Result<()> {
        let mut route_map: HashMap<(Method, String), HandlerEntry> = HashMap::new();
        let mut axum_router: AxumRouter<ServerState> = AxumRouter::new();

        // Track unique paths so we can auto-register OPTIONS handlers for
        // CORS preflights below.
        let mut paths_with_method: HashMap<String, HashSet<Method>> = HashMap::new();

        for route in &self.router.routes {
            let stacked = build_stack(route);
            let entry = HandlerEntry {
                handler: stacked,
                path_template: route.path.clone(),
            };
            route_map.insert((route.method, route.path.clone()), entry);
            paths_with_method.entry(route.path.clone()).or_default().insert(route.method);

            // Axum 0.7 uses `:param` already — no transform needed.
            let axum_path = route.path.clone();

            axum_router = match route.method {
                Method::GET => axum_router.route(&axum_path, get(dispatch)),
                Method::POST => axum_router.route(&axum_path, post(dispatch)),
                Method::PUT => axum_router.route(&axum_path, put(dispatch)),
                Method::PATCH => axum_router.route(&axum_path, patch(dispatch)),
                Method::DELETE => axum_router.route(&axum_path, delete(dispatch)),
                Method::OPTIONS => axum_router.route(&axum_path, options(dispatch)),
                _ => axum_router.route(&axum_path, get(dispatch)),
            };
        }

        // Auto-register OPTIONS handlers at every path that doesn't already
        // have one. The dispatcher will run any global middleware (CORS)
        // and short-circuit with 204; if no CORS middleware is registered,
        // we still answer the preflight with a vanilla 204.
        for (path, methods) in &paths_with_method {
            if !methods.contains(&Method::OPTIONS) {
                let preflight: crate::route::Handler = Arc::new(|_req| {
                    Box::pin(async { Ok(Response::no_content()) })
                });
                // Find any existing route at this path so we can mirror its
                // middleware stack (so CORS still runs).
                let middleware = self
                    .router
                    .routes
                    .iter()
                    .find(|r| &r.path == path)
                    .map(|r| r.middleware.clone())
                    .unwrap_or_default();
                let synthetic = crate::route::Route {
                    method: Method::OPTIONS,
                    path: path.clone(),
                    name: None,
                    middleware,
                    handler: preflight,
                };
                let stacked = build_stack(&synthetic);
                route_map.insert(
                    (Method::OPTIONS, path.clone()),
                    HandlerEntry { handler: stacked, path_template: path.clone() },
                );
                axum_router = axum_router.route(path, options(dispatch));
            }
        }

        let state = ServerState {
            routes: Arc::new(route_map),
        };
        let mut axum_router = axum_router.with_state(state);

        // SPA fallback: any unmatched request falls through to ServeDir.
        // Missing files within ServeDir fall back to index.html so the
        // client-side router handles deep links (e.g. /users/42).
        if let Some(dir) = self.public_dir.as_ref() {
            if dir.is_dir() {
                let index = dir.join("index.html");
                let serve_dir = ServeDir::new(dir).not_found_service(ServeFile::new(&index));
                axum_router = axum_router.fallback_service(serve_dir);
                tracing::info!("Serving static SPA from {}", dir.display());
            } else {
                tracing::warn!(
                    "public_dir {} not found — skipping static SPA fallback",
                    dir.display()
                );
            }
        }

        let listener = tokio::net::TcpListener::bind(&self.addr)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        tracing::info!("TARS server listening on {}", self.addr);
        axum::serve(listener, axum_router)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(())
    }
}

async fn dispatch(
    State(state): State<ServerState>,
    req: AxumRequest<Body>,
) -> std::result::Result<axum::response::Response, Error> {
    let method = match *req.method() {
        axum::http::Method::GET => Method::GET,
        axum::http::Method::POST => Method::POST,
        axum::http::Method::PUT => Method::PUT,
        axum::http::Method::PATCH => Method::PATCH,
        axum::http::Method::DELETE => Method::DELETE,
        axum::http::Method::OPTIONS => Method::OPTIONS,
        axum::http::Method::HEAD => Method::HEAD,
        _ => Method::GET,
    };

    // Find route by matching path pattern against the request URI.
    let path = req.uri().path().to_string();
    let mut matched: Option<(HandlerEntry, HashMap<String, String>)> = None;
    for ((m, template), entry) in state.routes.iter() {
        if *m == method {
            if let Some(params) = match_path(template, &path) {
                matched = Some((entry.clone(), params));
                break;
            }
        }
    }

    let (entry, params) = match matched {
        Some(v) => v,
        None => return Err(Error::NotFound),
    };

    let _ = entry.path_template;
    let mut tars_req = Request::from_axum(req, &()).await?;
    tars_req.route_params = params;

    let resp: Response = (entry.handler)(tars_req).await?;
    Ok(axum::response::IntoResponse::into_response(resp))
}

fn match_path(template: &str, path: &str) -> Option<HashMap<String, String>> {
    let t_segs: Vec<&str> = template.split('/').collect();
    let p_segs: Vec<&str> = path.split('/').collect();
    if t_segs.len() != p_segs.len() {
        return None;
    }
    let mut params = HashMap::new();
    for (t, p) in t_segs.iter().zip(p_segs.iter()) {
        if let Some(name) = t.strip_prefix(':') {
            params.insert(name.to_string(), p.to_string());
        } else if t != p {
            return None;
        }
    }
    Some(params)
}
