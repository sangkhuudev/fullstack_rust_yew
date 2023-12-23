use crate::components::{header::Header, sidebar::Sidebar, rustacean_list::RustaceanList};
use yew::prelude::*;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {
    html! {

        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <RustaceanList />
                </div>
            </div>
        </div>
    }
}
