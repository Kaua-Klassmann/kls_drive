use dioxus::prelude::*;

const LOGIN_CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        h1 { "Login" }
    }
}
