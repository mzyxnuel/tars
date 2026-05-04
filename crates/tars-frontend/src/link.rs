//! Vue-style `<router-link>` analogue. Renders an anchor tag and intercepts
//! click events to perform client-side navigation via the global router.

use dioxus::prelude::*;

use crate::router::navigate;

/// Props for `Link`. Mirrors Vue Router's `<router-link to="...">`.
#[derive(Props, PartialEq, Clone)]
pub struct LinkProps {
    pub to: String,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

/// `<Link to="/users">…</Link>` — client-side navigation without page reload.
#[component]
pub fn Link(props: LinkProps) -> Element {
    let to = props.to.clone();
    let to_for_click = to.clone();
    rsx! {
        a {
            href: "{to}",
            class: props.class.unwrap_or_default(),
            onclick: move |evt: Event<MouseData>| {
                evt.prevent_default();
                navigate(&to_for_click);
            },
            {props.children}
        }
    }
}
