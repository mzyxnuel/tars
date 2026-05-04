//! Boot entry point for the HTTP server. Analogous to Laravel's
//! `public/index.php` + `bootstrap/app.php`.

use example_app::routes;
use tars_core::Application;

#[tokio::main]
async fn main() -> tars_core::Result<()> {
    let mut app = Application::new();
    routes::web::register(&mut app.router);
    routes::api::register(&mut app.router);
    app.serve("0.0.0.0:8000").await
}
