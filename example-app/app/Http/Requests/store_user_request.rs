use async_trait::async_trait;
use tars_validation::FormRequest;

/// Form request for `POST /users`. Declares the same rule syntax as Laravel
/// — `required|email|max:255` etc.
pub struct StoreUserRequest;

#[async_trait]
impl FormRequest for StoreUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "required|string|min:2|max:100"),
            ("email", "required|email|max:255"),
        ]
    }
}
