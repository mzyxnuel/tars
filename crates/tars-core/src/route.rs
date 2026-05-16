use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::Result;
use crate::middleware::{Middleware, MiddlewareStack, Next};
use crate::request::Request;
use crate::response::Response;

/// Supported HTTP methods for route registration. Directly maps to axum's
/// method router when the router is built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    HEAD,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS",
            Method::HEAD => "HEAD",
        }
    }
}

/// A boxed controller-style handler. Accepts a `Request`, returns a `Response`.
pub type Handler = Arc<
    dyn Fn(Request) -> crate::middleware::BoxFuture<'static, Result<Response>> + Send + Sync,
>;

/// A compiled route — path, method, stack of middleware, and final handler.
#[derive(Clone)]
pub struct Route {
    pub method: Method,
    pub path: String,
    pub name: Option<String>,
    pub middleware: MiddlewareStack,
    pub handler: Handler,
}

impl Route {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn middleware(mut self, mw: Arc<dyn Middleware>) -> Self {
        self.middleware.push(mw);
        self
    }
}

/// Top-level router. Developers register routes via `get`, `post`, etc. and
/// it gets compiled into an axum Router when the application boots.
#[derive(Default, Clone)]
pub struct Router {
    pub routes: Vec<Route>,
    pub prefix: String,
    pub group_middleware: MiddlewareStack,
    pub named: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a handler for a given method + path pair.
    pub fn add<F, Fut>(&mut self, method: Method, path: &str, handler: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        let full_path = format!("{}{}", self.prefix, path);
        let handler: Handler = Arc::new(move |req| {
            let fut = handler(req);
            Box::pin(fut)
        });
        let route = Route {
            method,
            path: full_path,
            name: None,
            middleware: self.group_middleware.clone(),
            handler,
        };
        self.routes.push(route);
        self.routes.last_mut().unwrap()
    }

    pub fn get<F, Fut>(&mut self, path: &str, h: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        self.add(Method::GET, path, h)
    }

    pub fn post<F, Fut>(&mut self, path: &str, h: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        self.add(Method::POST, path, h)
    }

    pub fn put<F, Fut>(&mut self, path: &str, h: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        self.add(Method::PUT, path, h)
    }

    pub fn patch<F, Fut>(&mut self, path: &str, h: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        self.add(Method::PATCH, path, h)
    }

    pub fn delete<F, Fut>(&mut self, path: &str, h: F) -> &mut Route
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<Response>> + Send + 'static,
    {
        self.add(Method::DELETE, path, h)
    }

    /// Register the standard Laravel resource routes for a typed
    /// `Controller`. The router wires up route-model binding (`:id` →
    /// `Self::Model`) and form-request extraction automatically, so the
    /// controller methods only see the typed inputs they declare.
    pub fn resource<C: crate::controller::Controller>(&mut self, path: &str, controller: C) {
        use crate::binding::{Bindable, RouteBindable};
        use crate::error::Error;
        let base = path.trim_end_matches('/').to_string();

        // GET /resource → index
        let c = controller.clone();
        self.get(&base, move |_req| {
            let c = c.clone();
            async move { c.index().await }
        });

        // POST /resource → store (with FormRequest extraction)
        let c = controller.clone();
        self.post(&base, move |req| {
            let c = c.clone();
            async move {
                let form = <C::StoreRequest as Bindable>::from_request(&req).await?;
                c.store(form).await
            }
        });

        // GET /resource/:id → show (with route-model binding)
        let c = controller.clone();
        let show = format!("{}/:id", base);
        self.get(&show, move |req| {
            let c = c.clone();
            async move {
                let id = req.route("id").ok_or(Error::NotFound)?.to_string();
                let model = <C::Model as RouteBindable>::require_bind(&id).await?;
                c.show(model).await
            }
        });

        // PUT/PATCH /resource/:id → update (binding + FormRequest)
        let c = controller.clone();
        let update_path = format!("{}/:id", base);
        self.put(&update_path, move |req| {
            let c = c.clone();
            async move {
                let id = req.route("id").ok_or(Error::NotFound)?.to_string();
                let model = <C::Model as RouteBindable>::require_bind(&id).await?;
                let form = <C::UpdateRequest as Bindable>::from_request(&req).await?;
                c.update(model, form).await
            }
        });
        let c = controller.clone();
        let patch_path = format!("{}/:id", base);
        self.patch(&patch_path, move |req| {
            let c = c.clone();
            async move {
                let id = req.route("id").ok_or(Error::NotFound)?.to_string();
                let model = <C::Model as RouteBindable>::require_bind(&id).await?;
                let form = <C::UpdateRequest as Bindable>::from_request(&req).await?;
                c.update(model, form).await
            }
        });

        // DELETE /resource/:id → destroy
        let destroy_path = format!("{}/:id", base);
        self.delete(&destroy_path, move |req| {
            let c = controller.clone();
            async move {
                let id = req.route("id").ok_or(Error::NotFound)?.to_string();
                let model = <C::Model as RouteBindable>::require_bind(&id).await?;
                c.destroy(model).await
            }
        });
    }

    /// Start a prefix+middleware group — like Laravel's `Route::group(...)`.
    pub fn group<F>(&mut self, prefix: &str, f: F)
    where
        F: FnOnce(&mut Router),
    {
        let mut sub = Router {
            prefix: format!("{}{}", self.prefix, prefix),
            group_middleware: self.group_middleware.clone(),
            ..Default::default()
        };
        f(&mut sub);
        self.routes.extend(sub.routes);
        self.named.extend(sub.named);
    }

    /// Apply a middleware to every route currently registered, plus any
    /// future routes added through `group`. Equivalent to Laravel's
    /// `app('router')->aliasMiddleware` + global stack.
    pub fn apply_global(&mut self, mw: std::sync::Arc<dyn Middleware>) {
        for route in &mut self.routes {
            route.middleware.insert(0, mw.clone());
        }
        self.group_middleware.insert(0, mw);
    }
}

/// Build the final middleware-wrapped handler for a route. Applies middleware
/// in registration order so earlier middleware runs first.
pub fn build_stack(route: &Route) -> Handler {
    let mut handler = route.handler.clone();
    for mw in route.middleware.iter().rev().cloned() {
        let inner = handler.clone();
        handler = Arc::new(move |req| {
            let mw = mw.clone();
            let inner = inner.clone();
            Box::pin(async move {
                let next = Next::new(move |r| inner(r));
                mw.handle(req, next).await
            })
        });
    }
    handler
}

#[async_trait]
impl Middleware for () {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        next.run(req).await
    }
}
