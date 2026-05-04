//! Lightweight service provider analogue — no DI container in the MVP, but
//! this is where app-level bootstrap logic hooks in (eg. registering
//! singletons, event listeners, etc.).

pub struct AppServiceProvider;

impl AppServiceProvider {
    pub fn register() {
        // noop for MVP
    }

    pub fn boot() {
        // noop for MVP
    }
}
