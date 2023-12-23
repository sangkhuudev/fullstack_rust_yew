mod pages;
mod components;
mod api;
mod context;
mod hooks;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::navigator::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <context::CurrentUserProvider>
                <Switch<Route> render={switch} />
            </context::CurrentUserProvider>

        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
