use dioxus::prelude::*;

use crate::Route;

const LOGGED_IN_NAVBAR_CSS: Asset = asset!("/assets/styling/components/navbars.css");

#[component]
pub fn LoggedInNavbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGGED_IN_NAVBAR_CSS }
        nav { class: "navbar",
            Link { to: "/", class: "link", "Sair" }
        }

        Outlet::<Route> {}
    }
}
