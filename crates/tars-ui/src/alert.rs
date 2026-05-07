use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AlertVariant {
    Info,
    Success,
    Error,
    Warning,
}

impl AlertVariant {
    fn class(&self) -> &'static str {
        match self {
            AlertVariant::Info => "tars-alert tars-alert-info",
            AlertVariant::Success => "tars-alert tars-alert-success",
            AlertVariant::Error => "tars-alert tars-alert-error",
            AlertVariant::Warning => "tars-alert tars-alert-warning",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AlertProps {
    #[props(default = AlertVariant::Info)]
    pub variant: AlertVariant,
    pub children: Element,
}

#[component]
pub fn Alert(props: AlertProps) -> Element {
    rsx! { div { class: "{props.variant.class()}", {props.children} } }
}
