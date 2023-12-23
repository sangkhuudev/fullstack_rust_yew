use crate::components::{header::Header, sidebar::Sidebar, rustacean_form::RustaceanForm};
use yew::prelude::*;

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    html! {

        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <RustaceanForm />
                </div>
            </div>
        </div>
    }
}
