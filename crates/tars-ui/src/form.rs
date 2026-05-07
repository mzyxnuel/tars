use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FormGroupProps {
    pub children: Element,
}

/// Vertical stack of form fields with consistent spacing.
#[component]
pub fn FormGroup(props: FormGroupProps) -> Element {
    rsx! { div { class: "tars-form-group", {props.children} } }
}

#[derive(Props, PartialEq, Clone)]
pub struct FormFieldProps {
    pub label: String,
    #[props(default)]
    pub error: Option<String>,
    pub children: Element,
}

/// Label + control + (optional) error message — wraps any input.
#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    rsx! {
        div { class: "tars-form-field",
            label { class: "tars-label", "{props.label}" }
            {props.children}
            if let Some(err) = props.error.as_ref() {
                div { class: "tars-error", "{err}" }
            }
        }
    }
}
