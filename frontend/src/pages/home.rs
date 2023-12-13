use crate::components::{header::Header, sidebar::Sidebar};
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {

        <div class="container">
            <div class="row">
                <div class="col">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    {"Have a nice day"}
                </div>
            </div>
        </div>
    }
}
