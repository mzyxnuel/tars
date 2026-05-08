//! Backend route table. Mirrors Laravel's `routes/` directory, collapsed
//! into a single file since the framework is JSON-only — no need to
//! split web vs. api.

use tars_core::Router;

use crate::app::http::controllers::UserController;

pub fn register(router: &mut Router) {
    router.group("/api", |api| {
        api.resource("/users", UserController);
    });
}
