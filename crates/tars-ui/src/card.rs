use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    pub children: Element,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    rsx! {
        div { class: "tars-card {extra}", {props.children} }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CardHeaderProps {
    pub children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! { div { class: "tars-card-header", {props.children} } }
}

#[derive(Props, PartialEq, Clone)]
pub struct CardBodyProps {
    pub children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! { div { class: "tars-card-body", {props.children} } }
}
