//! TARS ORM — Eloquent-inspired data layer for Rust.
//!
//! Provides the `Model` trait, a `QueryBuilder`, and scaffolding for
//! migrations, seeders and factories. Uses sqlx underneath, so every
//! target database (SQLite, Postgres, MySQL) is supported.

pub mod connection;
pub mod factory;
pub mod migration;
pub mod model;
pub mod query;
pub mod seeder;

#[doc(hidden)]
pub mod __macro_support {
    // Re-export under a stable path so the `bind_model!` macro can reach
    // tars_core types without the user depending on it directly.
    pub use tars_core;
}

pub use connection::DB;
pub use factory::Factory;
pub use migration::{Migration, MigrationRunner, Schema};
pub use model::{Model, ModelInstance};
pub use query::QueryBuilder;
pub use seeder::Seeder;

pub mod prelude {
    pub use crate::{
        Factory, Migration, MigrationRunner, Model, ModelInstance, QueryBuilder, Schema, Seeder, DB,
    };
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
}
