//! Example TARS application — library crate that exposes the app modules
//! so the bootstrap binaries can import them. The directory layout mirrors
//! Laravel 13.

#[allow(non_snake_case)]
#[path = "../app/mod.rs"]
pub mod app;

#[path = "../models/mod.rs"]
pub mod models;

#[path = "../database/mod.rs"]
pub mod database;

#[path = "../routes/mod.rs"]
pub mod routes;
