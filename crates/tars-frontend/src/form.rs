//! Form state helpers — Vue-style v-model substitute. Avoids the
//! boilerplate of wiring up `value` + `oninput` for every field.

use dioxus::prelude::*;

/// Reactive form state for a single value. Returns the current value and
/// a setter, both signal-backed so they trigger re-renders.
///
/// ```ignore
/// let name = use_field("");
/// rsx! {
///     Input { value: name.get(), oninput: move |v| name.set(v) }
/// }
/// ```
pub fn use_field(initial: impl Into<String>) -> Field {
    let signal = use_signal(|| initial.into());
    Field { signal }
}

#[derive(Clone, Copy)]
pub struct Field {
    signal: Signal<String>,
}

impl Field {
    pub fn get(&self) -> String { self.signal.read().clone() }
    pub fn set(&mut self, v: impl Into<String>) { self.signal.set(v.into()); }
    pub fn signal(&self) -> Signal<String> { self.signal }
}

/// Reactive validation error map — typically populated from a 422
/// response body. Look up errors by field name.
#[derive(Clone, Copy)]
pub struct ValidationErrors {
    signal: Signal<serde_json::Value>,
}

pub fn use_validation_errors() -> ValidationErrors {
    ValidationErrors { signal: use_signal(|| serde_json::Value::Null) }
}

impl ValidationErrors {
    pub fn set(&mut self, errors: serde_json::Value) {
        self.signal.set(errors);
    }

    pub fn clear(&mut self) {
        self.signal.set(serde_json::Value::Null);
    }

    /// First error message for `field`, or `None`.
    pub fn first(&self, field: &str) -> Option<String> {
        let v = self.signal.read();
        v.get(field)
            .and_then(|f| f.as_array())
            .and_then(|arr| arr.first())
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }

    pub fn has_any(&self) -> bool {
        let v = self.signal.read();
        v.is_object() && !v.as_object().unwrap().is_empty()
    }
}
