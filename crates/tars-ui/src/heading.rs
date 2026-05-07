use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
}

#[derive(Props, PartialEq, Clone)]
pub struct HeadingProps {
    #[props(default = HeadingLevel::H1)]
    pub level: HeadingLevel,
    pub children: Element,
}

#[component]
pub fn Heading(props: HeadingProps) -> Element {
    match props.level {
        HeadingLevel::H1 => rsx! { h1 { class: "tars-h1", {props.children} } },
        HeadingLevel::H2 => rsx! { h2 { class: "tars-h2", {props.children} } },
        HeadingLevel::H3 => rsx! { h3 { class: "tars-h3", {props.children} } },
    }
}
