use yew::prelude::*;

use crate::components::sign_up_form::SignUpForm;

#[function_component]
pub fn SignUp() -> Html {
    html! {
        <div>
            <SignUpForm></SignUpForm>
        </div>
    }
}
