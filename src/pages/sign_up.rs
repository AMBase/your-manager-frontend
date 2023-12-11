use yew::prelude::*;
use log::debug;

use crate::components::sign_up_form::SignUpForm;
use crate::api::Api;

#[function_component]
pub fn SignUp() -> Html {
    let api: Api = use_context::<Api>().expect("no ctx found");

    debug!("{:?}", api);

    html! {
        <div>
            <SignUpForm></SignUpForm>
        </div>
    }
}
