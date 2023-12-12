use yew::prelude::*;
use yew_router::prelude::*;

use super::router::{switch, Route};
use crate::api::auth::Auth;
use crate::api::Api;

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(|| Api {
        host: "http://localhost:8000".to_string(),
        auth: Auth::new(),
    });

    html! {
        <ContextProvider<Api> context={(*ctx).clone()}>
            <main>
                <BrowserRouter>
                    <div>
                        <Link<Route> to={Route::Home}>{ "Home page" }</Link<Route>>
                        <Link<Route> to={Route::SignIn}>{ "SignIn page" }</Link<Route>>
                        <Link<Route> to={Route::SignUp}>{ "SignUp page" }</Link<Route>>
                    </div>

                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </BrowserRouter>
            </main>
        </ContextProvider<Api>>
    }
}
