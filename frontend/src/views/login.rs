use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    components::{Loading, MessageWithButtonLink},
    services,
};

const LOGIN_CSS: Asset = asset!("/assets/styling/views/login.css");

#[derive(Serialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[component]
pub fn Login() -> Element {
    //services::auth::drop();

    let backend_url = env!("BACKEND_URL");

    let mut message = use_signal(|| "".to_string());
    let mut success = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let submit = move |_: Event<MouseData>| async move {
        loading.set(true);

        message.set("".to_string());

        if email.read().len() == 0 || !email.read().contains("@") {
            loading.set(false);
            message.set("Email is required".to_string());
            return;
        }

        if password.read().len() < 6 {
            loading.set(false);
            message.set("Password must be at least 6 characters".to_string());
            return;
        }

        let payload = LoginPayload {
            email: email.read().clone(),
            password: password.read().clone(),
        };

        let response_result = reqwest::Client::new()
            .post(format!("{}/user/login", backend_url))
            .json(&payload)
            .send()
            .await;

        if response_result.is_err() {
            loading.set(false);
            message.set("Failed to connect to server".to_string());
            return;
        }

        let response = response_result.unwrap();

        if !response.status().is_success() {
            loading.set(false);
            message.set("Failed to login".to_string());
            return;
        }

        let login_response: LoginResponse = response.json().await.unwrap();

        services::auth::create(login_response.token);

        loading.set(false);
        success.set(true);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: LOGIN_CSS }
        div { id: "login_view",
            main { id: "login_form",
                h1 { "Login" }
                input {
                    r#type: "email",
                    placeholder: "Email",
                    oninput: move |evt| email.set(evt.value().clone()),
                    style: if success.read().to_owned() || loading.read().to_owned() { "cursor: not-allowed" } else { "" },
                    disabled: if success.read().to_owned() || loading.read().to_owned() { true } else { false },
                }
                input {
                    r#type: "password",
                    placeholder: "Password",
                    oninput: move |evt| password.set(evt.value().clone()),
                    style: if success.read().to_owned() || loading.read().to_owned() { "cursor: not-allowed" } else { "" },
                    disabled: if success.read().to_owned() || loading.read().to_owned() { true } else { false },
                }
                button {
                    onclick: submit,
                    style: if success.read().to_owned() || loading.read().to_owned() { "cursor: not-allowed" } else { "cursor: pointer" },
                    disabled: if success.read().to_owned() || loading.read().to_owned() { true } else { false },
                    "Create Account"
                }
                if message.read().len() > 0 {
                    ul { id: "error_messages",
                        li { "{message}" }
                    }
                }
            }
            if success.read().to_owned() {
                div { id: "div_message",
                    MessageWithButtonLink { message: "Login successful", url: "/home".to_string() }
                }
            }
            if loading.read().to_owned() {
                div { id: "div_loading", Loading {} }
            }
        }
    }
}
