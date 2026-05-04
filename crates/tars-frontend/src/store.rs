//! Vue-inspired reactivity primitives, implemented on top of Dioxus signals.
//!
//! - `ref_(value)` ≈ Vue `ref()` → returns a `Ref<T>` (Dioxus `Signal<T>`).
//! - `reactive(value)` ≈ Vue `reactive()` → same signal-based store but
//!   intended for struct-like state bags.

use dioxus::prelude::*;

/// Vue-style `ref` — a reactive single-value container backed by a Dioxus signal.
pub type Ref<T> = Signal<T>;

/// `ref(value)` equivalent. Must be called inside a component body
/// (Dioxus's hook contract).
#[allow(non_snake_case)]
pub fn ref_<T: 'static>(value: T) -> Ref<T> {
    use_signal(|| value)
}

/// `reactive(value)` equivalent — same as `ref_` but nominally used for
/// bigger objects. Returns the same underlying Signal.
pub fn reactive<T: 'static>(value: T) -> Ref<T> {
    use_signal(|| value)
}
