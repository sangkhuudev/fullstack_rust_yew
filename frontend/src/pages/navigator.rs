use yew::prelude::*;
use yew_router::prelude::*;
use super::home::Home;
use super::login::Login;
use super::not_found::NotFound;
use super::rustaceans::index::Rustaceans;
use super::rustaceans::add::RustaceansAdd;
use super::rustaceans::delete::RustaceansDelete;
use super::rustaceans::edit::RustaceansEdit;


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
    #[at("/rustaceans/:id/edit")]
    RustaceansEdit {id: i32},
    #[at("/rustaceans/:id/delete")]
    RustaceansDelete {id: i32},
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
        Route::RustaceansEdit {id} => html! {
            <RustaceansEdit  rustacean_id={id}/>
        },
        Route::RustaceansDelete {id} => html! {
            <RustaceansDelete  rustacean_id={id}/>
        },
        Route::NotFound => html! {
            <NotFound />
        },
        _ => html! {
            <Home />
        },
    }
}
