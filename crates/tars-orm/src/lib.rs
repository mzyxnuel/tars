//! TARS ORM — Eloquent-inspired data layer for Rust.
//!
//! Provides the `Model` trait, a `QueryBuilder`, and scaffolding for
//! migrations, seeders and factories. Uses sqlx underneath, so every target
//! database (SQLite, Postgres, MySQL) is supported.

pub mod connection;
pub mod factory;
pub mod migration;
pub mod model;
pub mod query;
pub mod resource;
pub mod seeder;

pub use connection::DB;
pub use factory::Factory;
pub use migration::{Migration, MigrationRunner, Schema};
pub use model::Model;
pub use query::QueryBuilder;
pub use resource::{JsonResource, ResourceCollection};
pub use seeder::Seeder;

pub mod prelude {
    pub use crate::{
        Factory, JsonResource, Migration, MigrationRunner, Model, QueryBuilder,
        ResourceCollection, Schema, Seeder, DB,
    };
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
}
