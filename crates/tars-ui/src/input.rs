use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    pub value: String,
    pub oninput: EventHandler<String>,
    #[props(default = "text".to_string())]
    pub r#type: String,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: Option<String>,
}

/// Controlled text input. Mirrors React/Vue's controlled-input pattern —
/// the parent owns the value and updates it via `oninput`.
#[component]
pub fn Input(props: InputProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    let class = format!("tars-input {}", extra);
    rsx! {
        input {
            class: "{class}",
            r#type: "{props.r#type}",
            value: "{props.value}",
            placeholder: props.placeholder.unwrap_or_default(),
            disabled: props.disabled,
            id: props.id.unwrap_or_default(),
            oninput: move |evt: FormEvent| props.oninput.call(evt.value()),
        }
    }
}
