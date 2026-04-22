use async_trait::async_trait;
use std::sync::Arc;

use crate::error::Result;
use crate::request::Request;
use crate::response::Response;

/// A Laravel-style middleware. Receives the request and a boxed `next`
/// closure that produces the downstream response. Either short-circuits
/// by returning early, or delegates to `next`.
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn handle(&self, req: Request, next: Next) -> Result<Response>;
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// The `next` callable passed into middleware — boxed so middleware can be
/// composed dynamically into a chain.
pub struct Next {
    pub(crate) inner: Box<dyn FnOnce(Request) -> BoxFuture<'static, Result<Response>> + Send>,
}

impl Next {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Request) -> BoxFuture<'static, Result<Response>> + Send + 'static,
    {
        Self { inner: Box::new(f) }
    }

    pub async fn run(self, req: Request) -> Result<Response> {
        (self.inner)(req).await
    }
}

pub type MiddlewareStack = Vec<Arc<dyn Middleware>>;
