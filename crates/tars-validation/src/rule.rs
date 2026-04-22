use regex::Regex;
use serde_json::Value;

/// Validation rule set. Mirrors the string-style rules developers write in
/// Laravel: `"required|email|max:255"`.
#[derive(Debug, Clone)]
pub enum Rule {
    Required,
    Nullable,
    String,
    Integer,
    Email,
    Min(i64),
    Max(i64),
    Regex(String),
    In(Vec<String>),
    Same(String),
    Boolean,
}

impl Rule {
    /// Validate the given value. Returns `None` on success or `Some(error)`.
    pub fn check(&self, field: &str, value: Option<&Value>, all: &Value) -> Option<String> {
        match self {
            Rule::Required => {
                if value.is_none() || matches!(value, Some(Value::Null)) || matches!(value, Some(Value::String(s)) if s.is_empty()) {
                    Some(format!("The {field} field is required."))
                } else {
                    None
                }
            }
            Rule::Nullable => None,
            Rule::String => match value {
                Some(Value::String(_)) | None | Some(Value::Null) => None,
                _ => Some(format!("The {field} must be a string.")),
            },
            Rule::Integer => match value {
                Some(Value::Number(n)) if n.is_i64() => None,
                None | Some(Value::Null) => None,
                _ => Some(format!("The {field} must be an integer.")),
            },
            Rule::Boolean => match value {
                Some(Value::Bool(_)) | None | Some(Value::Null) => None,
                _ => Some(format!("The {field} must be a boolean.")),
            },
            Rule::Email => match value {
                Some(Value::String(s)) => {
                    if s.contains('@') && s.contains('.') {
                        None
                    } else {
                        Some(format!("The {field} must be a valid email address."))
                    }
                }
                None | Some(Value::Null) => None,
                _ => Some(format!("The {field} must be a string.")),
            },
            Rule::Min(n) => match value {
                Some(Value::String(s)) if (s.chars().count() as i64) < *n => {
                    Some(format!("The {field} must be at least {n} characters."))
                }
                Some(Value::Number(num)) if num.as_i64().map(|i| i < *n).unwrap_or(false) => {
                    Some(format!("The {field} must be at least {n}."))
                }
                _ => None,
            },
            Rule::Max(n) => match value {
                Some(Value::String(s)) if (s.chars().count() as i64) > *n => {
                    Some(format!("The {field} may not be greater than {n} characters."))
                }
                Some(Value::Number(num)) if num.as_i64().map(|i| i > *n).unwrap_or(false) => {
                    Some(format!("The {field} may not be greater than {n}."))
                }
                _ => None,
            },
            Rule::Regex(pattern) => match value {
                Some(Value::String(s)) => match Regex::new(pattern) {
                    Ok(r) if r.is_match(s) => None,
                    Ok(_) => Some(format!("The {field} format is invalid.")),
                    Err(_) => Some(format!("The {field} has an invalid regex rule.")),
                },
                _ => None,
            },
            Rule::In(opts) => match value {
                Some(Value::String(s)) if opts.contains(s) => None,
                Some(Value::String(_)) => Some(format!("The selected {field} is invalid.")),
                _ => None,
            },
            Rule::Same(other) => {
                let a = value.cloned().unwrap_or(Value::Null);
                let b = all.get(other).cloned().unwrap_or(Value::Null);
                if a == b {
                    None
                } else {
                    Some(format!("The {field} and {other} must match."))
                }
            }
        }
    }
}

/// Parse a rule string like `"required|email|max:255"` into `Vec<Rule>`.
pub fn parse_rules(s: &str) -> Vec<Rule> {
    s.split('|')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            let mut split = part.splitn(2, ':');
            let name = split.next()?.to_ascii_lowercase();
            let arg = split.next().unwrap_or("");
            Some(match name.as_str() {
                "required" => Rule::Required,
                "nullable" => Rule::Nullable,
                "string" => Rule::String,
                "integer" | "int" => Rule::Integer,
                "boolean" | "bool" => Rule::Boolean,
                "email" => Rule::Email,
                "min" => Rule::Min(arg.parse().unwrap_or(0)),
                "max" => Rule::Max(arg.parse().unwrap_or(i64::MAX)),
                "regex" => Rule::Regex(arg.to_string()),
                "in" => Rule::In(arg.split(',').map(|s| s.trim().to_string()).collect()),
                "same" => Rule::Same(arg.to_string()),
                _ => return None,
            })
        })
        .collect()
}

/// Type alias for a rule map (field → list of rules).
pub type Rules = Vec<(String, Vec<Rule>)>;
