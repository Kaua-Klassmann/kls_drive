use dioxus::prelude::*;

use crate::{components::DocumentSubmit, services};

const HOME_CSS: Asset = asset!("assets/styling/views/home.css");

#[component]
pub fn Home() -> Element {
    services::auth::check();

    let mut add_file_button_clicked = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div { id: "home_view",
            div {
                id: "add_file_button",
                onclick: move |_| { add_file_button_clicked.toggle() },
                class: if *add_file_button_clicked.read() { "clicked" } else { "" },
                p { "+" }
            }
            if *add_file_button_clicked.read() {
                div { id: "document_submit", DocumentSubmit {} }
            }
        }
    }
}
