# TARS — Laravel-in-Rust

A Laravel-inspired full-stack web framework for Rust. v1.

TARS gives you Laravel's developer experience — controllers, migrations,
seeders, factories, form requests, resources, configs — but in Rust. The
HTTP layer wraps `axum`, the data layer wraps `sqlx`, the optional frontend
wraps `dioxus` (cross-compiles to web, desktop, and mobile), and the
component library `tars-ui` ships pre-styled Dioxus components.

- **JSON-first.** Frontend and backend always talk JSON.
- **Laravel 13 directory tree**, with two tweaks:
  - `/app` — controllers, middleware, requests, resources, providers
    *(no `app/Models` — see below)*
  - `/models` — all models, shared between frontend and backend
  - `/resources` — frontend source files (Vue-like components,
    file-based routes)

## Workspace layout

```
crates/
├── tars-core         HTTP, routing, request/response, config, CORS
├── tars-orm          Models, QueryBuilder, migrations, seeders, factories, resources
├── tars-validation   Validation rules + FormRequest
├── tars-ui           Dioxus component library (Button, Input, Card, Table, …)
├── tars-frontend     Dioxus wrapper with Vue-inspired DX + file-based routing
└── tars-cli          `tars` binary — artisan-like codegen + scaffold

example-app/          Full example with Laravel 13 directory tree
example-app/frontend/ Frontend binary that talks JSON to the backend
```

## Backend features

- `Application` + `Router` (groups, resource routes, global middleware)
- `Controller` + `Middleware` traits, async-trait based
- `Request` with `input`, `only`, `except`, `has`, `route`, `json` helpers
- `Response` with `json`, `created`, `no_content` helpers
- `config("app.name")` from `config/*.toml` files
- Models with `all`, `find`, `create`, `delete` + chainable `QueryBuilder`
- Migrations with fluent `Schema::create("users").id().string("name")`…
- Factories + Seeders
- Form requests (`StoreUserRequest::validated(&req).await?`) — Laravel
  rule strings: `"required|email|max:255"`
- Model resources (`UserResource::from_user(user).to_json()`)
- Built-in `Cors` middleware + auto OPTIONS preflight at every route

## Frontend features

- Optional — the backend has no dependency on `tars-frontend` / `tars-ui`
- Cross-compiles to **web / desktop / mobile** (anything Dioxus targets)
- Hooks: `use_signal()` (re-exported from Dioxus), `use_field()`,
  `use_validation_errors()`, `use_route_params()`, `use_router_path()`
- `<Link to="/users">…</Link>` for client-side navigation
- `use_router_path()`, `use_route_params()`, `navigate("/users")`
- File-based routing — drop a file in `resources/routes/`:
  - `index.rs` → `/`
  - `users.rs` → `/users`
  - `users/[id].rs` → `/users/:id`
  - `users/[id]/edit.rs` → `/users/:id/edit`
- `Api::default_base()` JSON client (gloo-net on web)
- `tars-ui` components (Button, Input, Card, Table, Alert, Form, …)
  with a single bundled stylesheet (`tars_ui::STYLES`)
- Reactive validation error map: `use_validation_errors()`

## Running the example app

Backend (terminal A):

```bash
cd example-app
cargo run --bin server
# 🚀 listens on 0.0.0.0:8000
# Migrations are auto-run on boot. SQLite file lives at storage/app/database.sqlite
```

Frontend (terminal B):

```bash
# Web (needs the dioxus CLI: `cargo install dioxus-cli`)
cd example-app/frontend
dx serve --features web

# Desktop (needs gtk + webkit2gtk dev libs on Linux)
cargo run -p example-app-frontend --features desktop
```

You'll see:
- `/` — landing page with cards
- `/users` — list of users with view / edit / delete actions
- `/users/create` — create form (with field-level validation errors)
- `/users/:id` — show page
- `/users/:id/edit` — edit form

All five pages use `tars-ui` components and talk to the backend over JSON.

## CLI (artisan-like)

```bash
tars new my-app                 # scaffold a project
tars make:controller PostController
tars make:model Post
tars make:migration create_posts_table
tars make:seeder PostSeeder
tars make:factory PostFactory
tars make:request StorePostRequest
tars make:resource PostResource
tars serve                      # cargo run --bin server
tars migrate                    # cargo run --bin migrate
tars db:seed                    # cargo run --bin seed
```

## Tests

```bash
cargo test --workspace
```

## License

MIT.
