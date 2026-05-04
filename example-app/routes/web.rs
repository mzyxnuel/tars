use tars_core::Router;
use tars_core::Response;
use serde_json::json;

/// Web routes — returns JSON for the MVP since the framework's transport
/// is JSON end-to-end. Frontend rendering is handled by `tars-frontend`.
pub fn register(router: &mut Router) {
    router.get("/", |_req| async move {
        Ok(Response::json(json!({
            "framework": "TARS",
            "message": "Welcome to TARS — Laravel-in-Rust."
        })))
    });
}
