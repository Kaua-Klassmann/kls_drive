use dioxus::prelude::*;

use crate::{services, AppState};

#[component]
pub fn Home() -> Element {
    services::auth::check();

    let app_state: Signal<AppState> = use_context();
    let token = app_state.read().token.clone();

    rsx! {
        div { "{token}" }
    }
}
