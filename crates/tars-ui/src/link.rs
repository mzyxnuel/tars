use dioxus::prelude::*;

/// Re-styled wrapper around `tars_frontend::Link`-equivalent that doesn't
/// pull `tars-frontend` as a dependency. Apps that already use
/// `tars_frontend::Link` should prefer that one — this is for projects
/// that want a styled `<a>` without the router glue.
#[derive(Props, PartialEq, Clone)]
pub struct AppLinkProps {
    pub href: String,
    #[props(default)]
    pub class: Option<String>,
    #[props(default = false)]
    pub active: bool,
    pub children: Element,
}

#[component]
pub fn AppLink(props: AppLinkProps) -> Element {
    let extra = props.class.clone().unwrap_or_default();
    let mut class = extra;
    if props.active { class.push_str(" active"); }
    rsx! {
        a { class: "{class}", href: "{props.href}", {props.children} }
    }
}
