use serde_json::{Map, Value};

use crate::validation::rule::{parse_rules, Rule};

/// Runs a rule set against a data payload. Returns either the validated
/// data or a map of field → error messages — formatted exactly like
/// Laravel's `ValidationException::errors()`.
pub struct Validator;

impl Validator {
    /// Validate `data` using the given rule set. Each tuple is
    /// `(field_name, rule_string)` where `rule_string` is the usual
    /// pipe-delimited Laravel format: `"required|email|max:255"`.
    pub fn validate(data: &Value, rules: &[(&str, &str)]) -> Result<Value, Value> {
        let mut errors: Map<String, Value> = Map::new();
        for (field, rule_str) in rules {
            let value = data.get(*field);
            for rule in parse_rules(rule_str) {
                if let Some(msg) = rule.check(field, value, data) {
                    errors
                        .entry(field.to_string())
                        .or_insert_with(|| Value::Array(vec![]))
                        .as_array_mut()
                        .unwrap()
                        .push(Value::String(msg));
                }
            }
        }
        if errors.is_empty() {
            Ok(data.clone())
        } else {
            Err(Value::Object(errors))
        }
    }

    /// Convenience: validate using already-parsed rules.
    pub fn validate_parsed(data: &Value, rules: &[(&str, Vec<Rule>)]) -> Result<Value, Value> {
        let mut errors: Map<String, Value> = Map::new();
        for (field, rs) in rules {
            let value = data.get(field);
            for r in rs {
                if let Some(msg) = r.check(field, value, data) {
                    errors
                        .entry(field.to_string())
                        .or_insert_with(|| Value::Array(vec![]))
                        .as_array_mut()
                        .unwrap()
                        .push(Value::String(msg));
                }
            }
        }
        if errors.is_empty() {
            Ok(data.clone())
        } else {
            Err(Value::Object(errors))
        }
    }
}
