use serde_json::json;
use tars_core::Validator;

#[test]
fn min_rule_errors_for_short_strings() {
    let payload = json!({ "password": "x" });
    let rules = vec![("password", "required|string|min:8")];
    assert!(Validator::validate(&payload, &rules).is_err());
}

#[test]
fn email_rule_catches_bad_emails() {
    let payload = json!({ "email": "not-an-email" });
    let rules = vec![("email", "required|email")];
    assert!(Validator::validate(&payload, &rules).is_err());
}
