use dioxus::prelude::*;

use crate::components::{LoggedInNavbar, LoggedOutNavbar};
use crate::views::{ActivateAccount, CreateAccount, Home, Login};

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
    #[layout(LoggedInNavbar)]
        #[route("/home")]
        Home {},
}
