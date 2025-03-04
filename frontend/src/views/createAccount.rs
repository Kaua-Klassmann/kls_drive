use dioxus::prelude::*;

const CREATE_ACCOUNT_CSS: Asset = asset!("/assets/styling/create_account.css");

#[component]
pub fn CreateAccount() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_ACCOUNT_CSS }
        div { id: "create_account_view",
            form {
                h1 { "Create Account" }
                input { r#type: "email", placeholder: "Email", required: true }
                input {
                    r#type: "password",
                    placeholder: "Password",
                    required: true,
                }
                button { "Create Account" }
            }
        }
    }
}
