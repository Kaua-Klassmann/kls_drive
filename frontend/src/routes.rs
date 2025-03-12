use dioxus::prelude::*;

use crate::components::LoggedOutNavbar;
use crate::views::{ActivateAccount, CreateAccount, Login, NotFound};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(LoggedOutNavbar)]
        #[route("/")]
        Login {},
        #[route("/account/register")]
        CreateAccount {},
        #[route("/activate/:activate_code")]
        ActivateAccount { activate_code: String },
    #[end_layout]
    #[route("/:_route")]
    NotFound { _route: String}
}
