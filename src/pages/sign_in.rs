use yew::prelude::*;

use crate::components::sign_in_form::SignInForm;

#[function_component]
pub fn SignIn() -> Html {
    html! {
        <div><SignInForm></SignInForm></div>
    }
}
