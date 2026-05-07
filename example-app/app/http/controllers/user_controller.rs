use tars_core::prelude::*;
use tars_orm::Model;
use tars_validation::FormRequest;

use crate::app::http::requests::{StoreUserRequest, UpdateUserRequest};
use crate::app::http::resources::UserResource;
use crate::models::User;

/// Users resource controller. Implements the seven Laravel resource
/// actions; stores records in the database via the `User` model.
#[derive(Clone, Default)]
pub struct UserController;

#[async_trait]
impl Controller for UserController {
    async fn index(&self, _req: Request) -> Result<Response> {
        let users = User::query()
            .order_by("id", false)
            .get()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;
        let body: Vec<Value> = users.into_iter().map(|u| UserResource::from_user(u).to_json()).collect();
        Ok(Response::json(json!({ "data": body })))
    }

    async fn show(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        let user = User::find(id)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?
            .ok_or(Error::NotFound)?;
        Ok(Response::json(json!({ "data": UserResource::from_user(user).to_json() })))
    }

    async fn store(&self, req: Request) -> Result<Response> {
        let validated = StoreUserRequest::validated(&req).await?;
        let mut payload = validated.clone();
        let now = chrono::Utc::now().to_rfc3339();
        if let Some(map) = payload.as_object_mut() {
            map.insert("created_at".into(), Value::String(now.clone()));
            map.insert("updated_at".into(), Value::String(now));
        }
        User::create(payload).await.map_err(|e| Error::Internal(e.to_string()))?;

        // Return the freshly inserted row (most recently created).
        let created = User::query()
            .order_by("id", false)
            .limit(1)
            .first()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?
            .ok_or_else(|| Error::Internal("Failed to fetch created user".into()))?;
        Ok(Response::created(json!({ "data": UserResource::from_user(created).to_json() })))
    }

    async fn update(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        let validated = UpdateUserRequest::validated(&req).await?;

        let mut sets = vec![];
        let mut binds: Vec<serde_json::Value> = vec![];
        if let Some(obj) = validated.as_object() {
            for (k, v) in obj.iter() {
                sets.push(format!("{} = ${}", k, binds.len() + 1));
                binds.push(v.clone());
            }
        }
        if sets.is_empty() {
            return Err(Error::BadRequest("No updatable fields provided".into()));
        }
        sets.push(format!("updated_at = ${}", binds.len() + 1));
        binds.push(Value::String(chrono::Utc::now().to_rfc3339()));

        let id_index = binds.len() + 1;
        binds.push(tars_orm::model::coerce_id(&id));
        let sql = format!("UPDATE users SET {} WHERE id = ${}", sets.join(", "), id_index);

        let mut q = sqlx::query::<sqlx::Any>(&sql);
        for v in binds {
            q = tars_orm::model::bind_value(q, v);
        }
        let _: sqlx::any::AnyQueryResult = q
            .execute(tars_orm::DB::pool())
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        let user = User::find(id)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?
            .ok_or(Error::NotFound)?;
        Ok(Response::json(json!({ "data": UserResource::from_user(user).to_json() })))
    }

    async fn destroy(&self, req: Request) -> Result<Response> {
        let id = req.route("id").ok_or(Error::NotFound)?.to_string();
        User::delete(id).await.map_err(|e| Error::Internal(e.to_string()))?;
        Ok(Response::no_content())
    }
}
