use dioxus::prelude::*;
use serde::Serialize;

use crate::components::MessageWithButtonLink;

const CREATE_ACCOUNT_CSS: Asset = asset!("/assets/styling/views/create_account.css");

#[derive(Serialize)]
struct RegisterUserPayload {
    email: String,
    password: String,
}

#[component]
pub fn CreateAccount() -> Element {
    let backend_url = env!("BACKEND_URL");

    let mut messages = use_signal(|| Vec::<String>::new());
    let mut success = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let submit = move |_: Event<MouseData>| async move {
        loading.set(true);

        messages.set(Vec::new());

        if email.read().len() == 0 || !email.read().contains("@") {
            loading.set(false);
            messages.write().push("Email is required".to_string());
            return;
        }

        if password.read().len() < 6 {
            loading.set(false);
            messages
                .write()
                .push("Password must be at least 6 characters".to_string());
            return;
        }

        let payload = RegisterUserPayload {
            email: email.read().clone(),
            password: password.read().clone(),
        };

        let response_result = reqwest::Client::new()
            .post(format!("{}/user/register", backend_url))
            .json(&payload)
            .send()
            .await;

        if response_result.is_err() {
            loading.set(false);
            messages
                .write()
                .push("Failed to connect to server".to_string());
            return;
        }

        let response = response_result.unwrap();

        if !response.status().is_success() {
            loading.set(false);
            messages
                .write()
                .push("Failed to create account".to_string());
            return;
        }

        loading.set(false);
        success.set(true);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_ACCOUNT_CSS }
        div { id: "create_account_view",
            main { id: "create_account_form",
                h1 { "Create Account" }
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
                if messages.read().len() > 0 {
                    ul { id: "error_messages",
                        for message in messages.read().iter() {
                            li { "{message}" }
                        }
                    }
                }
            }
        }
        if success.read().to_owned() {
            MessageWithButtonLink { message: "Email sent to {email}", url: "/".to_string() }
        }
        if loading.read().to_owned() {
            div { id: "loading" }
        }
    }
}
