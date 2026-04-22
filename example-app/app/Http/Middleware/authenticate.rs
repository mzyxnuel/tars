use tars_core::prelude::*;
use tars_core::middleware::Next;

/// Example authentication middleware. Rejects requests missing an
/// `Authorization` header. Illustrates the Next/handle pattern.
pub struct Authenticate;

#[async_trait]
impl Middleware for Authenticate {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        if req.header("authorization").is_none() {
            return Err(Error::Unauthorized);
        }
        next.run(req).await
    }
}
