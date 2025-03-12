use dioxus::prelude::*;

#[component]
pub fn NotFound(_route: String) -> Element {
    use_navigator().push("/");

    rsx! {}
}
