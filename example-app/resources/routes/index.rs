//! `/` — landing page. Shows quick links into the Users module.

use dioxus::prelude::*;
use tars_frontend::Link;
use tars_ui::prelude::*;

pub fn component() -> Element {
    rsx! {
        Container {
            Page { title: "Welcome to TARS".to_string(),
                div { class: "tars-stack",
                    Alert { variant: AlertVariant::Info,
                        "TARS is a Laravel-inspired framework for Rust. The frontend you're "
                        "looking at is a Dioxus app sharing models with the backend over JSON."
                    }
                    div { class: "tars-grid",
                        Card {
                            CardHeader { Heading { level: HeadingLevel::H3, "Users" } }
                            CardBody {
                                p { class: "tars-muted",
                                    "Browse, create, edit, and delete users. Demonstrates the full CRUD lifecycle."
                                }
                                div { style: "margin-top: 12px;",
                                    Link { to: "/users".to_string(),
                                        Button { variant: ButtonVariant::Primary, "Open users" }
                                    }
                                }
                            }
                        }
                        Card {
                            CardHeader { Heading { level: HeadingLevel::H3, "Stack" } }
                            CardBody {
                                ul { class: "tars-muted",
                                    li { "Backend: tars-core + tars-orm (axum, sqlx)" }
                                    li { "Validation: tars-validation FormRequests" }
                                    li { "Frontend: tars-frontend (Dioxus + file-based routing)" }
                                    li { "UI: tars-ui component library" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
