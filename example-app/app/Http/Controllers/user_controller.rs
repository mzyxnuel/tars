use tars_core::prelude::*;
use tars_orm::Model;
use tars_validation::FormRequest;

use crate::app::Http::Requests::StoreUserRequest;
use crate::app::Http::Resources::UserResource;
use crate::models::User;

/// Users controller — demonstrates index/store/show/update/destroy plus
/// form request validation and a model resource for the output shape.
#[derive(Clone, Default)]
pub struct UserController;

#[async_trait]
impl Controller for UserController {
    async fn index(&self, _req: Request) -> Result<Response> {
        let users = User::all().await.map_err(|e| Error::Internal(e.to_string()))?;
        let resources: Vec<_> = users.into_iter().map(UserResource::from_user).collect();
        let body: Vec<Value> = resources.iter().map(|r| r.to_json()).collect();
        Ok(Response::json(json!({ "data": body })))
    }

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
        User::create(validated.clone()).await.map_err(|e| Error::Internal(e.to_string()))?;
        Ok(Response::created(json!({ "message": "User created", "user": validated })))
    }

    async fn update(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        Ok(Response::json(json!({ "updated": id, "payload": req.body })))
    }

    async fn destroy(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        User::delete(id).await.map_err(|e| Error::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }
}
