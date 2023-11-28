use yew::prelude::*;

#[function_component]
pub fn SignInForm() -> Html {
    html! {
        <form>
            <input name="email" />

            <button>{ "Отправить" }</button>
        </form>
    }
}
