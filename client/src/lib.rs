use axum::response::Html;
use dioxus::prelude::Writable;
use dioxus::prelude::*;
use dioxus::{core_macro::rsx, dioxus_core::Element, hooks::use_signal};

pub fn app() -> Element {
    // let mut user_name = use_signal(|| "?".to_string());
    // let mut permissions = use_signal(|| "?".to_string());

    rsx! {
        div {
            "Hello world!"
        }
    }
}
