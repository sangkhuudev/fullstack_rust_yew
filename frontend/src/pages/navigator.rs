use yew::prelude::*;
use yew_router::prelude::*;
use super::home::Home;
use super::login::Login;
use super::not_found::NotFound;
use super::rustaceans::index::Rustaceans;
use super::rustaceans::add::RustaceansAdd;


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
    #[at("/rustaceans/add")]
    RustaceansAdd,
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
        Route::Rustaceans => html! {
            <Rustaceans />
        },
        Route::RustaceansAdd => html! {
            <RustaceansAdd />
        },
        Route::NotFound => html! {
            <NotFound />
        },
        _ => html! {
            <Home />
        },
    }
}
