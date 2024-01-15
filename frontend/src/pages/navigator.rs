use yew::prelude::*;
use yew_router::prelude::*;
use super::crates::add::CratesAdd;
use super::crates::delete::CratesDelete;
use super::crates::edit::CratesEdit;
use super::crates::index::Crates;
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
    #[at("/crates/add")]
    CratesAdd,
    #[at("/crates/:id/edit")]
    CratesEdit {id: i32},
    #[at("/crates/:id/delete")]
    CratesDelete {id: i32},
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
        Route::Crates => html! {
            <Crates />
        },
        Route::CratesAdd => html! {
            <CratesAdd  />
        },
        Route::CratesEdit {id} => html! {
            <CratesEdit crate_id={id}/>
        },
        Route::CratesDelete {id} => html! {
            <CratesDelete crate_id={id}/>
        },
    }
}
