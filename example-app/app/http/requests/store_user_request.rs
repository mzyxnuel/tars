use serde::{Deserialize, Serialize};
use tars_core::FormRequest;

/// Form request for `POST /users`. Fields hold the validated payload —
/// access them directly (`req.name`) or call `req.validated()` for the
/// raw JSON shape ready to pass to `Model::create`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreUserRequest {
    pub name: String,
    pub email: String,
}

impl FormRequest for StoreUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "required|string|min:2|max:100"),
            ("email", "required|email|max:255"),
        ]
    }
}
