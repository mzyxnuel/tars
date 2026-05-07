use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ContainerProps {
    pub children: Element,
    #[props(default)]
    pub class: Option<String>,
}

/// Centered, max-width container — Laravel's `<main class="container">`.
#[component]
pub fn Container(props: ContainerProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    rsx! {
        div { class: "tars-container {extra}", {props.children} }
    }
}
