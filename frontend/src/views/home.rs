use dioxus::prelude::*;

use crate::services;

#[component]
pub fn Home() -> Element {
    services::auth::check();

    let token = services::auth::get();

    rsx! {
        div { "{token}" }
    }
}
