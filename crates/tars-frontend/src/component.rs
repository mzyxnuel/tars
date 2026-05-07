//! Component primitives.
//!
//! `Component` is a marker trait implemented by component structs that
//! prefer a typed-Props style. Most apps will write Dioxus `#[component]`
//! functions directly — this trait is provided for the cases where a
//! struct-shaped surface reads better.

use dioxus::prelude::*;

/// Marker trait — implement on a struct that knows how to render itself.
pub trait Component {
    type Props: ComponentProps;

    /// Render this component with the given props.
    fn render(props: Self::Props) -> Element;
}

/// Props trait — `()` works for components that take nothing.
pub trait ComponentProps: Clone + PartialEq + 'static {}

impl ComponentProps for () {}

/// Define a zero-arg render function as a component without ceremony.
/// Equivalent to writing the function directly; kept for symmetry with
/// the rest of the API.
pub fn define_component<F>(f: F) -> F
where
    F: Fn() -> Element,
{
    f
}
