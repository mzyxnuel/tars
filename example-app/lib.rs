//! Example TARS application — library crate that exposes the app modules
//! so the bootstrap binaries can import them. The directory layout mirrors
//! Laravel 13 (snake_case'd for Rust).

#[path = "app/mod.rs"]
pub mod app;

#[path = "models/mod.rs"]
pub mod models;

#[path = "database/mod.rs"]
pub mod database;

pub mod routes;
