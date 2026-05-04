use axum::body::Body;
use axum::extract::State;
use axum::http::Request as AxumRequest;
use axum::routing::{delete, get, patch, post, put};
use axum::Router as AxumRouter;
use std::collections::HashMap;
use std::sync::Arc;

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
}

impl Server {
    pub fn new(addr: impl Into<String>, router: Router) -> Self {
        Self { addr: addr.into(), router }
    }

    pub async fn run(self) -> Result<()> {
        let mut route_map: HashMap<(Method, String), HandlerEntry> = HashMap::new();
        let mut axum_router: AxumRouter<ServerState> = AxumRouter::new();

        for route in &self.router.routes {
            let stacked = build_stack(route);
            let entry = HandlerEntry {
                handler: stacked,
                path_template: route.path.clone(),
            };
            route_map.insert((route.method, route.path.clone()), entry);

            // Convert `:param` → `{param}` for axum 0.7
            let axum_path = route
                .path
                .split('/')
                .map(|seg| {
                    if let Some(p) = seg.strip_prefix(':') {
                        format!("{{{}}}", p)
                    } else {
                        seg.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("/");

            axum_router = match route.method {
                Method::GET => axum_router.route(&axum_path, get(dispatch)),
                Method::POST => axum_router.route(&axum_path, post(dispatch)),
                Method::PUT => axum_router.route(&axum_path, put(dispatch)),
                Method::PATCH => axum_router.route(&axum_path, patch(dispatch)),
                Method::DELETE => axum_router.route(&axum_path, delete(dispatch)),
                _ => axum_router.route(&axum_path, get(dispatch)),
            };
        }

        let state = ServerState {
            routes: Arc::new(route_map),
        };
        let axum_router = axum_router.with_state(state);

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
