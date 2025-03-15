use dioxus::prelude::*;

use crate::AppState;

pub fn create(token: String) {
    let mut app_state: Signal<AppState> = use_context();

    app_state.write().token = token;
}

pub fn get() -> String {
    let app_state: Signal<AppState> = use_context();

    let token = app_state.read().token.clone();

    token
}

pub fn check() {
    let app_state: Signal<AppState> = use_context();

    if app_state.read().token == *"".to_string() {
        use_navigator().go_back();
    }
}
