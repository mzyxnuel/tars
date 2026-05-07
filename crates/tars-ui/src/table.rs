use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TableProps { pub children: Element }
#[component]
pub fn Table(props: TableProps) -> Element {
    rsx! { table { class: "tars-table", {props.children} } }
}

#[derive(Props, PartialEq, Clone)]
pub struct TheadProps { pub children: Element }
#[component]
pub fn Thead(props: TheadProps) -> Element {
    rsx! { thead { tr { {props.children} } } }
}

#[derive(Props, PartialEq, Clone)]
pub struct TrProps { pub children: Element }
#[component]
pub fn Tr(props: TrProps) -> Element {
    rsx! { tr { {props.children} } }
}

#[derive(Props, PartialEq, Clone)]
pub struct ThProps { pub children: Element }
#[component]
pub fn Th(props: ThProps) -> Element {
    rsx! { th { {props.children} } }
}

#[derive(Props, PartialEq, Clone)]
pub struct TdProps { pub children: Element }
#[component]
pub fn Td(props: TdProps) -> Element {
    rsx! { td { {props.children} } }
}
