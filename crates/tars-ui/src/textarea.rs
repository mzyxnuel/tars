use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextareaProps {
    pub value: String,
    pub oninput: EventHandler<String>,
    #[props(default)]
    pub placeholder: Option<String>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub id: Option<String>,
    #[props(default = 4)]
    pub rows: i64,
}

#[component]
pub fn Textarea(props: TextareaProps) -> Element {
    rsx! {
        textarea {
            class: "tars-textarea",
            value: "{props.value}",
            placeholder: props.placeholder.unwrap_or_default(),
            disabled: props.disabled,
            id: props.id.unwrap_or_default(),
            rows: props.rows,
            oninput: move |evt: FormEvent| props.oninput.call(evt.value()),
        }
    }
}
