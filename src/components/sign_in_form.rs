use log::info;
use web_sys::{EventTarget, HtmlInputElement};
use yew::events::SubmitEvent;
use yew::prelude::*;

#[function_component]
pub fn SignInForm() -> Html {
    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let target: EventTarget = e.target().unwrap();

        info!("{target:?}");
    });

    html! {
        <form onsubmit={on_submit}>
            <input name="email" />

            <button>{ "Отправить" }</button>
        </form>
    }
}
