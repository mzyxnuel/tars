use dioxus::prelude::*;

/// Visual style for `Button`.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "tars-btn tars-btn-primary",
            ButtonVariant::Secondary => "tars-btn tars-btn-secondary",
            ButtonVariant::Danger => "tars-btn tars-btn-danger",
            ButtonVariant::Ghost => "tars-btn tars-btn-ghost",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[props(default = "button".to_string())]
    pub r#type: String,
    #[props(default = false)]
    pub disabled: bool,
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

/// Styled, accessible button. Defaults to `type="button"` so it never
/// implicitly submits forms — pass `r#type: "submit"` when you want it to.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    let class = format!("{} {}", props.variant.class(), extra);
    rsx! {
        button {
            class: "{class}",
            r#type: "{props.r#type}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(h) = &props.onclick {
                    h.call(evt);
                }
            },
            {props.children}
        }
    }
}
