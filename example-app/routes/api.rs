use tars_core::Router;

use crate::app::http::controllers::UserController;

/// Register the API route table. Mirrors Laravel's `routes/api.php`.
pub fn register(router: &mut Router) {
    router.group("/api", |api| {
        api.resource("/users", UserController);
    });
}
