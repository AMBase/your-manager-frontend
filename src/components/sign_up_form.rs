use crate::api::Api;
use log::debug;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::events::SubmitEvent;
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInData {
    pub email: String,
}

#[function_component]
pub fn SignUpForm() -> Html {
    let api: Api = use_context::<Api>().expect("no ctx found");

    debug!("{:?}", api);

    let input_email = use_node_ref();
    let on_submit = {
        let api = api.clone();
        let input_email = input_email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let api = api.clone();
            let target: EventTarget = e.target().unwrap();
            let email_value = input_email.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let resp = api.auth.sign_up(email_value).await;
                debug!("{:?}", resp);
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input name="email" ref={input_email.clone()} />

            <button>{ "Отправить" }</button>
        </form>
    }
}
