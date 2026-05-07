//! Cross-target launcher. Picks the right Dioxus renderer based on which
//! feature is enabled at compile time. Mirrors Laravel's `php artisan serve`
//! convenience but for the frontend.

use dioxus::prelude::*;

/// Boot the Dioxus app. Selects the renderer based on the active feature:
/// `web`, `desktop`, `mobile`, or `fullstack`. Without any of those features
/// this becomes a no-op so library users can still build & test the app
/// without a renderer installed.
#[allow(unused_variables)]
pub fn launch(root: fn() -> Element) {
    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile", feature = "fullstack"))]
    {
        dioxus::launch(root);
        return;
    }
    #[cfg(not(any(feature = "web", feature = "desktop", feature = "mobile", feature = "fullstack")))]
    {
        eprintln!(
            "tars_frontend::launch was called but no renderer feature is enabled. \
             Enable one of: web, desktop, mobile, fullstack."
        );
    }
}
