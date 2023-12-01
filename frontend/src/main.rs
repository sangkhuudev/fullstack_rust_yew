mod pages;
mod components;
mod api;
// mod components;
// mod pages;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::navigator::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
