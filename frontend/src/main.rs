#![allow(non_snake_case)]

use dioxus::prelude::*;

use components::LoggedOutNavbar;
use views::{ActivateAccount, CreateAccount, Login};

mod components;
mod views;

#[derive(Clone)]
pub struct AppState {
    token: String,
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LoggedOutNavbar)]
        #[route("/")]
        Login {},
        #[route("/account/register")]
        CreateAccount {},
        #[route("/activate/:activate_code")]
        ActivateAccount { activate_code: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

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
