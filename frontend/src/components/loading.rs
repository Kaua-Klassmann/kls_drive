use dioxus::prelude::*;

const LOADING_CSS: Asset = asset!("/assets/styling/components/loading.css");

#[component]
pub fn Loading() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOADING_CSS }
        div { id: "loading" }
    }
}
