use log::{debug, info, trace};
use web_sys::EventTarget;
use yew::events::SubmitEvent;
use yew::prelude::*;

use crate::api::Api;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::debug;
use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode, Response};

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
        let input_email = input_email.clone();
        let api = api.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let target: EventTarget = e.target().unwrap();
            let email_value = input_email.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let resp = api.auth.sign_up(email_value).await;
                debug!("{:?}", resp);

                // // Convert this other `Promise` into a rust `Future`.
                // let json = JsFuture::from(resp.json()?).await?;
                // info!("{json:?}");
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
