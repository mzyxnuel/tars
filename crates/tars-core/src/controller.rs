use async_trait::async_trait;

use crate::binding::{Bindable, RouteBindable};
use crate::error::{Error, Result};
use crate::response::Response;

/// Resource controller. Each action takes only the typed parameters it
/// actually needs: a bound `Model` instance for `:id` routes and/or a
/// `FormRequest` for actions with a request body.
///
/// ```ignore
/// impl Controller for UserController {
///     type Model = User;
///     type StoreRequest = StoreUserRequest;
///     type UpdateRequest = UpdateUserRequest;
///
///     async fn index(&self) -> Result<Response> {
///         Ok(UserResource::collection(User::all().await?))
///     }
///
///     async fn show(&self, user: User) -> Result<Response> {
///         Ok(UserResource::single(user))
///     }
///
///     async fn store(&self, req: StoreUserRequest) -> Result<Response> {
///         Ok(UserResource::created(User::create(req.validated()).await?))
///     }
///
///     async fn update(&self, user: User, req: UpdateUserRequest) -> Result<Response> {
///         Ok(UserResource::single(user.update(req.validated()).await?))
///     }
///
///     async fn destroy(&self, user: User) -> Result<Response> {
///         user.delete().await?;
///         Ok(Response::no_content())
///     }
/// }
/// ```
#[async_trait]
pub trait Controller: Clone + Send + Sync + 'static {
    /// The model class the `:id` path parameter resolves to. The router
    /// calls `Model::route_bind(id)` for `show`/`update`/`destroy`. Use
    /// [`NoModel`](crate::binding::NoModel) if the controller has no
    /// resource-style routes.
    type Model: RouteBindable;

    /// Form request decoded from the body for `store`. Set to `()` if
    /// the controller's `store` doesn't take a body.
    type StoreRequest: Bindable;

    /// Form request decoded from the body for `update`. Set to `()` if
    /// the controller's `update` doesn't take a body.
    type UpdateRequest: Bindable;

    async fn index(&self) -> Result<Response> {
        Err(Error::NotFound)
    }

    async fn show(&self, _model: Self::Model) -> Result<Response> {
        Err(Error::NotFound)
    }

    async fn store(&self, _req: Self::StoreRequest) -> Result<Response> {
        Err(Error::NotFound)
    }

    async fn update(
        &self,
        _model: Self::Model,
        _req: Self::UpdateRequest,
    ) -> Result<Response> {
        Err(Error::NotFound)
    }

    async fn destroy(&self, _model: Self::Model) -> Result<Response> {
        Err(Error::NotFound)
    }
}
