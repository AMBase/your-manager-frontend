use yew::prelude::*;
use yew_router::prelude::*;



#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign_in")]
    SignIn,
    #[at("/sign_up")]
    SignUp,
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <div>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>


        </div>
    }
}
#[function_component]
pub fn SignIn() -> Html {
    html! {
        <div>{ "Page SignIn" }</div>
    }
}

#[function_component]
pub fn SignUp() -> Html {
    html! {
        <div>{ "Page SignUp" }</div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home></Home> },
        Route::SignIn => html! { <SignIn></SignIn> },
        Route::SignUp => html! { <SignUp></SignUp> },
    }
}

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

