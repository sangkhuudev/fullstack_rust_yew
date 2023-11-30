mod pages;
mod components;
mod api;

use yew::prelude::*;
use pages::login::{Login};


#[function_component(App)]
fn app() -> Html {
    html! {
        <Login />
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
