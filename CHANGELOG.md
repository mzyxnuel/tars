# Changelog

All notable changes to the TARS framework are recorded here. Follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0]

**Breaking** — this release reshapes the controller / form-request /
resource surface. Use it as a checklist when upgrading other projects.

### Crate layout

- **Removed** the `tars-validation` crate. Its contents (`Validator`,
  `Rule`, `parse_rules`, `FormRequest`) now live in
  `tars_core::validation::*` and are re-exported from the `tars_core`
  prelude. Folding them into `tars-core` was required to make the
  `FormRequest` → `Bindable` blanket impl legal under Rust's orphan
  rule.
  - Migration: drop the `tars-validation` dependency from your
    `Cargo.toml` and rewrite `use tars_validation::*` →
    `use tars_core::*`.
- The `JsonResource` trait moved from `tars-orm` to `tars-core` (it's
  about HTTP responses, not the DB). Adjust imports accordingly.
- `tars-orm` now depends on `tars-core` so it can interact with the
  framework's binding traits.

### Controller — typed signatures + automatic dispatch

The `Controller` trait now has three associated types — `Model`,
`StoreRequest`, `UpdateRequest` — and the action methods take only the
typed inputs they need. The router fetches the bound model and validates
form requests automatically; controllers never call `req.route("id")`
or `req.validated()` for raw extraction anymore.

**Before:**

```rust
#[async_trait]
impl Controller for UserController {
    async fn show(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        let user = User::find(id)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?
            .ok_or(Error::NotFound)?;
        Ok(Response::json(UserResource::from_user(user).to_json()))
    }

    async fn store(&self, req: Request) -> Result<Response> {
        let validated = StoreUserRequest::validated(&req).await?;
        User::create(validated.clone()).await?;
        // refetch the latest row…
        Ok(Response::created(json!({ "data": … })))
    }

    async fn update(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        let validated = UpdateUserRequest::validated(&req).await?;
        // build dynamic UPDATE SQL manually…
        // refetch the row…
        Ok(Response::json(…))
    }
}
```

**After:**

```rust
#[async_trait]
impl Controller for UserController {
    type Model = User;
    type StoreRequest = StoreUserRequest;
    type UpdateRequest = UpdateUserRequest;

    async fn index(&self) -> Result<Response> {
        Ok(UserResource::collection(User::all().await?))
    }
    async fn show(&self, user: User) -> Result<Response> {
        Ok(UserResource::single(user))
    }
    async fn store(&self, req: StoreUserRequest) -> Result<Response> {
        Ok(UserResource::created(User::create(req.validated()).await?))
    }
    async fn update(&self, user: User, req: UpdateUserRequest) -> Result<Response> {
        Ok(UserResource::single(user.update(req.validated()).await?))
    }
    async fn destroy(&self, user: User) -> Result<Response> {
        user.delete().await?;
        Ok(Response::no_content())
    }
}
```

Migration checklist:

- Add `type Model = …;`, `type StoreRequest = …;`, `type UpdateRequest = …;`
  to every `impl Controller for …`. Use `NoModel` / `()` for actions
  that don't have a resource / body.
- Drop the `Request` parameter from every action signature; take the
  typed model and/or form request instead.
- Replace `let id = req.route("id")?; let model = Model::find(id)?…`
  with the bound parameter (`user: User`). The framework returns 404
  automatically when the lookup misses.

### Route-model binding (`RouteBindable`)

- `tars-core` ships a `RouteBindable` trait that the router uses to
  resolve `:id` segments. `Controller::Model: RouteBindable`.
- `tars-orm` provides a `bind_model!(Type)` `macro_rules!` that
  generates the impl by delegating to `Model::find`. Invoke it once
  per model:

  ```rust
  impl Model for User { fn table() -> &'static str { "users" } }
  tars_orm::bind_model!(User);
  ```

  (Couldn't be a blanket impl due to the orphan rule, so it's expanded
  in the user's crate.)
- `NoModel` exists as a sentinel for controllers that don't have a
  resource concept.

### `FormRequest` carries the validated data

- The trait now requires `Serialize + DeserializeOwned`. The struct
  itself holds the validated payload — declare data fields directly.
- The framework calls `<T as Bindable>::from_request(req)` for you when
  a controller action declares a `FormRequest` parameter; manual
  `T::validated(&req).await?` calls are no longer needed.
- `FormRequest::validated(&self) -> Value` returns the JSON shape ready
  to feed into `Model::create(req.validated())`.

**Before:**

```rust
pub struct StoreUserRequest;
#[async_trait]
impl FormRequest for StoreUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> { … }
}

// in the controller:
let validated = StoreUserRequest::validated(&req).await?;
```

**After:**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreUserRequest {
    pub name: String,
    pub email: String,
}

impl FormRequest for StoreUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> { … }
}

// in the controller:
async fn store(&self, req: StoreUserRequest) -> Result<Response> {
    User::create(req.validated()).await?;
    …
}
```

For optional/partial payloads (`update`), override `validated()` to
drop `None` fields so they don't overwrite existing columns with NULL —
see `UpdateUserRequest` in the example app.

### `Model::create` / `update_by_id` return `Self`

- `Model::create(payload) -> Result<Self>` (was `Result<u64>`). Uses
  `INSERT ... RETURNING *` so the new row — including DB-side defaults
  and the auto-incremented id — is returned in a single round-trip.
  Works on **SQLite 3.35+** and **Postgres**; MySQL 8 has no
  `RETURNING` and needs further work.
- `Model::delete` renamed to **`Model::delete_by_id`**. There's also a
  new instance method `model.delete().await?` via the new
  `ModelInstance` extension trait.
- New **`Model::update_by_id(id, payload)`** plus instance-side
  `model.update(payload).await?` — both use `RETURNING *` to return the
  refreshed model.
- New **`Model::uses_timestamps()`** (defaults to `true`). When `true`,
  `create` auto-fills `created_at`/`updated_at` and `update_by_id` (and
  `model.update`) auto-bumps `updated_at`. Override to opt out for
  tables without those columns.
- `Factory::create_many` returns `Vec<Self::M>` instead of `u64`.

### `JsonResource` constructors

The trait grew helpers that return a fully formed `Response`:

- `JsonResource::single(model)` — wraps a single model in
  `{"data": {…}}` and returns a `Response`.
- `JsonResource::collection(models)` — maps each model and wraps the
  array in `{"data": […]}`.
- `JsonResource::created(model)` — same as `single` but with a 201
  status.

The associated type was renamed from `M` to `Model` and `from_user`
(example-app's specific name) is now `from_model` (the trait method).

**Before:**

```rust
let users = User::all().await?;
let body: Vec<Value> = users
    .into_iter()
    .map(|u| UserResource::from_user(u).to_json())
    .collect();
Ok(Response::json(json!({ "data": body })))
```

**After:**

```rust
Ok(UserResource::collection(User::all().await?))
```

### Misc

- `tars-cli`'s `make:controller`, `make:request`, `make:resource`
  templates emit code in the new shape.

[2.0.0]: https://github.com/mzyxnuel/tars/releases/tag/v2.0.0
