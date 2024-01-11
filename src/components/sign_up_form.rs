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
    let input_password = use_node_ref();
    let input_password_confirm = use_node_ref();
    let on_submit = {
        let api = api.clone();
        let navigator = navigator.clone();

        let input_email = input_email.clone();
        let input_password = input_password.clone();
        let input_password_confirm = input_password_confirm.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let api = api.clone();
            let navigator = navigator.clone();
            let target: EventTarget = e.target().unwrap();
            let html_input_email: HtmlInputElement = input_email.cast().unwrap();
            let html_input_password: HtmlInputElement = input_password.cast().unwrap();
            let html_input_password_confirm: HtmlInputElement =
                input_password_confirm.cast().unwrap();
            let email_value = html_input_email.value();
            let password_value = html_input_password.value();
            let password_confirm_value = html_input_password_confirm.value();

            spawn_local(async move {
                let resp = api
                    .auth
                    .signup(email_value, password_value, password_confirm_value)
                    .await;
                html_input_email.set_value("");

                match resp.status() {
                    200 | 201 | 405 => navigator.push(&Route::SignUpSuccess),
                    _ => debug!(""),
                }
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <input name="email" ref={input_email.clone()} />
            <input name="password" ref={input_password.clone()} />
            <input name="password_confirm" ref={input_password_confirm.clone()} />

            <button>{ "Отправить" }</button>
        </form>
    }
}
