use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PageProps {
    pub title: String,
    #[props(default)]
    pub actions: Option<Element>,
    pub children: Element,
}

/// Conventional page shell — heading + optional action bar + body.
#[component]
pub fn Page(props: PageProps) -> Element {
    rsx! {
        div { class: "tars-page",
            div { class: "tars-page-header",
                h1 { class: "tars-h1", "{props.title}" }
                if let Some(actions) = props.actions {
                    div { class: "tars-page-actions", {actions} }
                }
            }
            {props.children}
        }
    }
}
