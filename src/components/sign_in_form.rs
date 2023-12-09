use log::{debug, info, trace};
use web_sys::EventTarget;
use yew::events::SubmitEvent;
use yew::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInData {
    pub email: String,
}

#[function_component]
pub fn SignInForm() -> Html {
    let input_email = use_node_ref();
    let on_submit = {
        let input_email = input_email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let target: EventTarget = e.target().unwrap();
            let email_value = input_email.cast::<HtmlInputElement>().unwrap().value();

            spawn_local(async move {
                let url = format!("/api/v1/auth/signin");
                let mut opts = RequestInit::new();
                opts.method("POST");
                opts.mode(RequestMode::Cors);
                let v = json!({"email": email_value}).to_string();
                let jsv = JsValue::from_str(&v);
                opts.body(Some(&jsv));
                let request = Request::new_with_str_and_init(&url, &opts).unwrap();

                let _ = request.headers().append("Accept", "application/json");
                let window = web_sys::window().unwrap();
                let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
                let resp: Response = resp_value.unwrap().dyn_into().unwrap();
                debug!("{:?}", resp.status());

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
