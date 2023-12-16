use crate::api::Api;
use crate::router::Route;

use log::debug;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::console::debug;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::events::SubmitEvent;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInData {
    pub email: String,
}

#[function_component]
pub fn SignUpForm() -> Html {
    let api: Api = use_context::<Api>().expect("no ctx found");
    let navigator = use_navigator().unwrap();

    debug!("{:?}", api);

    let input_email = use_node_ref();
    let on_submit = {
        let api = api.clone();
        let navigator = navigator.clone();

        let input_email = input_email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let api = api.clone();
            let navigator = navigator.clone();
            let target: EventTarget = e.target().unwrap();
            let input: HtmlInputElement = input_email.cast().unwrap();
            let email_value = input.value();

            spawn_local(async move {
                let resp = api.auth.sign_up(email_value).await;
                input.set_value("");

                match resp.status() {
                    200 | 201 => navigator.push(&Route::Home),
                    _ => debug!(""),
                }
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
