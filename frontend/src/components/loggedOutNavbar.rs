use dioxus::prelude::*;

use crate::Route;

const LOGGED_OUT_NAVBAR_CSS: Asset = asset!("/assets/styling/components/logged_out_navbar.css");

#[component]
pub fn LoggedOutNavbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGGED_OUT_NAVBAR_CSS }
        nav { class: "navbar",
            Link { to: "/", class: "link", "login" }
            Link { to: "/account/register", class: "link", "register" }
        }

        Outlet::<Route> {}
    }
}
