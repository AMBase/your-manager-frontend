use log::debug;
use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Clone, Debug, PartialEq)]
pub struct Auth {}

impl Auth {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn signin(self, email_value: String) -> Response {
        let url = format!("/api/v1/auth/signin");
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        let password = "password".to_string();
        let password_confirm = "password_confirm".to_string();
        let v = json!({"email": email_value, "password": password, "password_confirm": password_confirm}).to_string();
        let jsv = JsValue::from_str(&v);
        opts.body(Some(&jsv));
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let _ = request.headers().append("Accept", "application/json");
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
        let resp: Response = resp_value.unwrap().into();

        resp
    }

    pub async fn signup(
        self,
        email: String,
        password: String,
        password_confirm: String,
    ) -> Response {
        let url = format!("/api/v1/auth/signup");
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        let v = json!({"email": email, "password": password, "password_confirm": password_confirm})
            .to_string();
        let jsv = JsValue::from_str(&v);
        opts.body(Some(&jsv));
        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let _ = request.headers().append("Accept", "application/json");
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
        let resp: Response = resp_value.unwrap().into();

        resp
    }
}
