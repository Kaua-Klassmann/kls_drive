#![allow(non_snake_case)]

use dioxus::prelude::*;

use routes::Route;

mod components;
mod routes;
mod services;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Clone)]
pub struct AppState {
    token: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| {
        Signal::new(AppState {
            token: "".to_string(),
        })
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
