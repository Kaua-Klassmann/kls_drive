use dioxus::prelude::*;
use reqwest::StatusCode;

use crate::components::{Loading, MessageWithButtonLink};

const ACTIVATE_ACCOUNT_CSS: Asset = asset!("/assets/styling/views/activate_account.css");

#[component]
pub fn ActivateAccount(activate_code: String) -> Element {
    let backend_url = env!("BACKEND_URL");

    let mut error = use_signal(|| false);
    let mut success = use_signal(|| false);

    use_effect(move || {
        to_owned![activate_code];

        spawn(async move {
            let response_result = reqwest::Client::new()
                .get(format!("{}/user/activate/{}", backend_url, activate_code))
                .send()
                .await;

            if response_result.is_err() {
                error.set(true);
            }

            let response = response_result.unwrap();

            if response.status() != StatusCode::OK {
                error.set(true);
            }

            success.set(true);
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: ACTIVATE_ACCOUNT_CSS }
        div { id: "activate_view",
            div { id: "activate_box",
                h1 { "Activating Account" }
                div { id: "div_loading", Loading {} }
            }
            if success.read().to_owned() {
                div { id: "div_message",
                    MessageWithButtonLink {
                        message: "Account has been actived",
                        url: "/".to_string(),
                    }
                }
            }
            if error.read().to_owned() {
                div { id: "div_message",
                    MessageWithButtonLink {
                        message: "Failed to activate account".to_string(),
                        url: "/account/register".to_string(),
                    }
                }
            }
        }
    }
}
