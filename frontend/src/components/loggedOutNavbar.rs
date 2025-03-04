use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn LoggedOutNavbar() -> Element {
    rsx! {
        nav { "LoggedOutNavbar" }

        Outlet::<Route> {}
    }
}
