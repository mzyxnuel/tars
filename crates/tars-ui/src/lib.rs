//! `tars-ui` — Dioxus component library for the TARS framework.
//!
//! Inspired by shadcn/ui + headlessui — gives you a sane set of styled
//! components that work across web, desktop, and mobile (anything Dioxus
//! can render). Every component takes a `class` prop so you can extend or
//! override the bundled styles, and the whole stylesheet is exposed via
//! [`STYLES`] so a single `style { {STYLES} }` mount is all you need.

mod alert;
mod button;
mod card;
mod container;
mod form;
mod heading;
mod input;
mod link;
mod page;
mod spinner;
mod styles;
mod table;
mod textarea;

pub use alert::{Alert, AlertVariant};
pub use button::{Button, ButtonVariant};
pub use card::{Card, CardBody, CardHeader};
pub use container::Container;
pub use form::{FormField, FormGroup};
pub use heading::{Heading, HeadingLevel};
pub use input::Input;
pub use link::AppLink;
pub use page::Page;
pub use spinner::Spinner;
pub use styles::STYLES;
pub use table::{Table, Td, Th, Thead, Tr};
pub use textarea::Textarea;

pub mod prelude {
    pub use crate::{
        Alert, AlertVariant, AppLink, Button, ButtonVariant, Card, CardBody, CardHeader,
        Container, FormField, FormGroup, Heading, HeadingLevel, Input, Page, Spinner, Table, Td,
        Textarea, Th, Thead, Tr, STYLES,
    };
}
