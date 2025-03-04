use dioxus::prelude::*;

const CREATE_ACCOUNT_CSS: Asset = asset!("/assets/styling/create_account.css");

#[component]
pub fn CreateAccount() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_ACCOUNT_CSS }
        h1 { "Create Account" }
    }
}
