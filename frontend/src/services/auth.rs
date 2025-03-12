use dioxus::prelude::*;

use crate::AppState;

pub fn create(token: String) {
    let mut app_state: Signal<AppState> = use_context();

    app_state.write().token = token;
}
