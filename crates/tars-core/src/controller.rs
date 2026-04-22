use async_trait::async_trait;

use crate::error::Result;
use crate::request::Request;
use crate::response::Response;

/// Trait implemented by every controller. The seven methods match Laravel's
/// resource controller surface — developers only implement what they need
/// and leave the rest as the default 404.
#[async_trait]
pub trait Controller: Send + Sync + 'static {
    async fn index(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn create(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn store(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn show(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn edit(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn update(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }

    async fn destroy(&self, _req: Request) -> Result<Response> {
        Err(crate::error::Error::NotFound)
    }
}
