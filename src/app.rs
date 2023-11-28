use yew::prelude::*;
use yew_router::prelude::*;

use super::router::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
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
    }
}
