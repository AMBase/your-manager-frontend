use log::info;
use web_sys::EventTarget;
use yew::events::SubmitEvent;
use yew::prelude::*;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInData {
    pub email: String,
}

#[function_component]
pub fn SignInForm() -> Html {
    let input_email = NodeRef::default();
    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let target: EventTarget = e.target().unwrap();

        spawn_local(async {
            let url = format!("/api/v1/auth/signin");
            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            let v = r#"
                {
                    "email": "test@example.com"
                }
            "#;
            let jsv = JsValue::from_str(v);
            opts.body(Some(&jsv));
            let request = Request::new_with_str_and_init(&url, &opts).unwrap();

            request.headers().append("Accept", "application/json");

            println!("{request:?}");

            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;

            // // `resp_value` is a `Response` object.
            // assert!(resp_value.is_instance_of::<Response>());
            // let resp: Response = resp_value.dyn_into().unwrap();
            //
            // // Convert this other `Promise` into a rust `Future`.
            // let json = JsFuture::from(resp.json()?).await?;
            // info!("{json:?}");
        });
    });

    html! {
        <form onsubmit={on_submit}>
            <input name="email" ref={input_email.clone()} />

            <button>{ "Отправить" }</button>
        </form>
    }
}
