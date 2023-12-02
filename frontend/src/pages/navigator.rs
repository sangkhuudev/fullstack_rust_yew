use yew::prelude::*;
use yew_router::prelude::*;
use super::home::Home;
use super::login::Login;
use super::not_found::NotFound;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/crates")]
    Crates,
    #[at("/rustaceans")]
    Rustaceans,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::Login => html! {
            <Login />
        },
        Route::NotFound => html! {
            <NotFound />
        },
        _ => html! {
            <Home />
        },
    }
}
