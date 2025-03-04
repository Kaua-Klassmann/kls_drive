use dioxus::prelude::*;

use components::LoggedOutNavbar;
use views::{CreateAccount, Login};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LoggedOutNavbar)]
        #[route("/")]
        Login {},
        #[route("/account/register")]
        CreateAccount {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        Router::<Route> {}
    }
}
