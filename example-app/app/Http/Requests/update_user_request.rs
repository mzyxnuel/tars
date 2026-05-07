use async_trait::async_trait;
use tars_validation::FormRequest;

/// Form request for `PUT /users/:id`. Same rule shape as store, but
/// fields are optional — partial updates are allowed.
pub struct UpdateUserRequest;

#[async_trait]
impl FormRequest for UpdateUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "nullable|string|min:2|max:100"),
            ("email", "nullable|email|max:255"),
        ]
    }
}
