use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    rsx! { span { class: "tars-spinner", aria_label: "loading" } }
}
