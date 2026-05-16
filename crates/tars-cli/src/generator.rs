use anyhow::Result;
use std::fs;
use std::path::Path;

fn write(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if path.exists() {
        anyhow::bail!("{} already exists", path.display());
    }
    fs::write(path, contents)?;
    println!("  created {}", path.display());
    Ok(())
}

pub fn make_controller(name: &str) -> Result<()> {
    let path = format!("app/http/controllers/{}.rs", to_snake(name));
    let src = format!(
        r#"use tars_core::prelude::*;

#[derive(Clone, Default)]
pub struct {name};

#[async_trait]
impl Controller for {name} {{
    // Set these to the right concrete types for your resource — e.g.
    // `type Model = Post;` and a `StoreRequest`/`UpdateRequest` impl.
    type Model = NoModel;
    type StoreRequest = ();
    type UpdateRequest = ();

    async fn index(&self) -> Result<Response> {{
        Ok(Response::json(json!({{ "data": [] }})))
    }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_model(name: &str) -> Result<()> {
    let table = pluralize(&to_snake(name));
    let path = format!("models/{}.rs", to_snake(name));
    let src = format!(
        r#"use serde::{{Deserialize, Serialize}};
use tars_orm::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {name} {{
    pub id: Option<i64>,
    // TODO: add columns
}}

impl Model for {name} {{
    fn table() -> &'static str {{ "{table}" }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_migration(name: &str) -> Result<()> {
    let ts = chrono::Utc::now().format("%Y_%m_%d_%H%M%S");
    let snake = to_snake(name);
    let struct_name = to_pascal(name);
    let path = format!("database/migrations/{ts}_{snake}.rs");
    let src = format!(
        r#"use tars_orm::{{Migration, Schema}};
use async_trait::async_trait;

pub struct {struct_name};

#[async_trait]
impl Migration for {struct_name} {{
    fn name(&self) -> &'static str {{ "{ts}_{snake}" }}

    async fn up(&self) -> Result<(), sqlx::Error> {{
        Schema::create("table_name")
            .id()
            .timestamps()
            .execute()
            .await
    }}

    async fn down(&self) -> Result<(), sqlx::Error> {{
        Schema::drop("table_name").await
    }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_seeder(name: &str) -> Result<()> {
    let path = format!("database/seeders/{}.rs", to_snake(name));
    let src = format!(
        r#"use tars_orm::Seeder;
use async_trait::async_trait;

pub struct {name};

#[async_trait]
impl Seeder for {name} {{
    fn name(&self) -> &'static str {{ "{name}" }}

    async fn run(&self) -> Result<(), sqlx::Error> {{
        Ok(())
    }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_factory(name: &str) -> Result<()> {
    let path = format!("database/factories/{}.rs", to_snake(name));
    let src = format!(
        r#"use serde_json::json;
use tars_orm::Factory;
use async_trait::async_trait;

pub struct {name};

#[async_trait]
impl Factory for {name} {{
    type M = (); // Replace with the target Model type

    fn definition(&self) -> serde_json::Value {{
        json!({{}})
    }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_request(name: &str) -> Result<()> {
    let path = format!("app/http/requests/{}.rs", to_snake(name));
    let src = format!(
        r#"use serde::{{Deserialize, Serialize}};
use tars_core::FormRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {name} {{
    // TODO: add fields matching your rules
}}

impl FormRequest for {name} {{
    fn rules() -> Vec<(&'static str, &'static str)> {{
        vec![
            // ("email", "required|email"),
        ]
    }}
}}
"#
    );
    write(Path::new(&path), &src)
}

pub fn make_resource(name: &str) -> Result<()> {
    let path = format!("app/http/resources/{}.rs", to_snake(name));
    let src = format!(
        r#"use serde_json::{{json, Value}};
use tars_core::JsonResource;

pub struct {name}<M> {{ pub model: M }}

impl<M: Send + Sync + 'static> JsonResource for {name}<M> {{
    type Model = M;
    fn from_model(model: M) -> Self {{ Self {{ model }} }}
    fn to_json(&self) -> Value {{ json!({{}}) }}
}}
"#
    );
    write(Path::new(&path), &src)
}

fn to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

fn to_pascal(s: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

fn pluralize(s: &str) -> String {
    if s.ends_with('s') { s.to_string() } else { format!("{s}s") }
}
