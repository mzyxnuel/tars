use tars_core::prelude::*;
use tars_orm::{Model, ModelInstance};

use crate::app::http::requests::{StoreUserRequest, UpdateUserRequest};
use crate::app::http::resources::UserResource;
use crate::models::User;

/// Users resource controller. Each action takes only what it needs:
/// - `index` — nothing (lists every row)
/// - `show` / `destroy` — a `User` already resolved from `:id`
/// - `store` — a validated `StoreUserRequest`
/// - `update` — both the bound user and a validated `UpdateUserRequest`
///
/// `:id` ⇒ `User` happens via the framework's route-model binding (every
/// `Model` implements `RouteBindable`), and validation runs automatically
/// when a method declares a `FormRequest` parameter.
#[derive(Clone, Default)]
pub struct UserController;

#[async_trait]
impl Controller for UserController {
    type Model = User;
    type StoreRequest = StoreUserRequest;
    type UpdateRequest = UpdateUserRequest;

    async fn index(&self) -> Result<Response> {
        let users = User::query()
            .order_by("id", false)
            .get()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(UserResource::collection(users))
    }

    async fn show(&self, user: User) -> Result<Response> {
        Ok(UserResource::single(user))
    }

    async fn store(&self, req: StoreUserRequest) -> Result<Response> {
        let user = User::create(req.validated())
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(UserResource::created(user))
    }

    async fn update(&self, user: User, req: UpdateUserRequest) -> Result<Response> {
        let user = user
            .update(req.validated())
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(UserResource::single(user))
    }

    async fn destroy(&self, user: User) -> Result<Response> {
        user.delete()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }
}
