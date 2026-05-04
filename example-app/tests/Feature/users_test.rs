//! Feature test for the users endpoint. Uses the validator directly rather
//! than booting the HTTP server — this keeps the MVP test fast and
//! dependency-free.

use serde_json::json;
use tars_validation::Validator;

#[test]
fn store_user_request_rules_reject_missing_email() {
    let payload = json!({ "name": "Ada" });
    let rules = vec![
        ("name", "required|string|min:2|max:100"),
        ("email", "required|email|max:255"),
    ];
    let result = Validator::validate(&payload, &rules);
    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert!(errors.get("email").is_some());
}

#[test]
fn store_user_request_rules_accept_valid_payload() {
    let payload = json!({ "name": "Ada", "email": "ada@example.com" });
    let rules = vec![
        ("name", "required|string|min:2|max:100"),
        ("email", "required|email|max:255"),
    ];
    assert!(Validator::validate(&payload, &rules).is_ok());
}
