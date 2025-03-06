use dioxus::prelude::*;

const LOGIN_CSS: Asset = asset!("/assets/styling/views/login.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        div { id: "login_view",
            form { id: "login_form",
                h1 { "Login" }
                input { r#type: "email", placeholder: "Email", required: true }
                input {
                    r#type: "password",
                    placeholder: "Password",
                    required: true,
                }
                button { "Login" }
            }
        }
    }
}
