# TARS

A Laravel-inspired web framework for Rust. MVP.

TARS brings Laravel's developer experience — controllers, migrations,
seeders, factories, form requests, resources, configs — to Rust. The HTTP
layer wraps `axum`, the data layer wraps `sqlx`, and the optional frontend
wraps `dioxus` (cross-compiles to web, desktop, and mobile).

- **JSON first.** Frontend and backend always talk JSON.
- **Laravel 13 directory tree**, with a couple of tweaks:
  - `/app` — controllers, middleware, requests, resources, providers
    *(no `app/Models` — see below)*
  - `/models` — all models, shared between frontend and backend
  - `/resources` — frontend source files (Vue-like components,
    file-based routes)

## Workspace layout

```
crates/
├── tars-core         HTTP, routing, request/response, config
├── tars-orm          Models, QueryBuilder, migrations, seeders, factories, resources
├── tars-validation   Validation rules, FormRequest
├── tars-cli          `tars` binary — artisan-like codegen + scaffold
└── tars-frontend     Dioxus wrapper with Vue-inspired DX + file-based routing

example-app/          Full example with Laravel 13 directory tree
```

## Backend features

- Application + Router + Controller + Middleware
- Request with `input`, `only`, `except`, `has`, `route`, `json` helpers
- Response with `json`, `created`, `no_content` helpers
- Resource routing (`router.resource("/users", UserController)`)
- Route grouping with prefix + middleware
- Configuration via `config/*.toml` files and `config("app.name")` helper
- Models with `all`, `find`, `create`, `delete` + chainable `QueryBuilder`
- Migrations with a fluent `Schema::create("users").id().string("name")...`
- Factories + Seeders
- Form requests: `StoreUserRequest::validated(&req).await?`
- Model resources: `UserResource::from_user(user).to_json()`

## Frontend features

- Optional — the backend has no dependency on `tars-frontend`
- Dioxus wrapper, so any Dioxus target works (web / desktop / mobile)
- Vue-inspired: `defineComponent`, `ref_()`, `reactive()`
- `<Link to="/users">…</Link>` for client-side navigation
- `use_router_path()` hook for reactive route tracking
- `navigate("/users")` for imperative routing
- File-based routing (`resources/routes/users.rs` → `/users`,
  `resources/routes/users/[id].rs` → `/users/:id`) — discovered at build
  time by `build.rs` (no manual route registration)
- `Api::new("/api")` JSON client (uses `gloo-net` on the web target)
- Shared model types via `/models`

### Running the frontend

The example app ships a frontend crate at `example-app/frontend/`:

```bash
# desktop target (requires gtk/webkit dev libraries)
cargo run -p example-app-frontend --features desktop

# web target — use `dx serve` from the dioxus CLI
cd example-app/frontend && dx serve --features web
```

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

## Getting started

```bash
cargo build --workspace
cd example-app
cargo test --test validation_unit
cargo test --test users_feature
```

The example app shows every scaffolding piece working end-to-end.
