//! Vue-inspired component primitive.
//!
//! `Component` is a trait implemented by component structs. Under the hood
//! it's just a Dioxus component function, but the struct-based API keeps
//! things familiar for developers coming from Vue's Options/Composition API.

use dioxus::prelude::*;

/// Marker trait — every TARS component implements this.
pub trait Component {
    type Props: ComponentProps;

    /// Equivalent of Vue's `setup()` / render function. Returns a Dioxus
    /// `Element`.
    fn render(props: Self::Props) -> Element;
}

/// Props trait — `()` is acceptable for components that take nothing.
pub trait ComponentProps: Clone + PartialEq + 'static {}

impl ComponentProps for () {}

/// Helper matching Vue's `defineComponent({ name, setup })` DX. Wraps a
/// plain render function into a zero-sized component type.
#[allow(non_snake_case)]
pub fn defineComponent<F>(f: F) -> F
where
    F: Fn() -> Element,
{
    f
}
