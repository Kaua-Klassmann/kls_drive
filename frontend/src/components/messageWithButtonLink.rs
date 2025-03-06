use dioxus::prelude::*;

const MESSAGE_WITH_BUTTON_LINK_CSS: Asset =
    asset!("/assets/styling/components/message_with_button_link.css");

#[component]
pub fn MessageWithButtonLink(message: String, url: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MESSAGE_WITH_BUTTON_LINK_CSS }
        div { id: "success_message",
            h1 { "{message}" }
            Link { to: "{url}", class: "link",
                button { "OK" }
            }
        }
    }
}
