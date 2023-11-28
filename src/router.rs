use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Home, SignIn, SignUp};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign_in")]
    SignIn,
    #[at("/sign_up")]
    SignUp,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home></Home> },
        Route::SignIn => html! { <SignIn></SignIn> },
        Route::SignUp => html! { <SignUp></SignUp> },
    }
}
