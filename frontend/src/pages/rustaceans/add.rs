use crate::{components::{header::Header, sidebar::Sidebar, rustacean_form::RustaceanForm}, api::rustaceans::Rustacean};
use yew::prelude::*;

#[function_component(RustaceansAdd)]
pub fn rustaceans_add() -> Html {
    // let sample_rustacean = Rustacean {
    //     id: 0,
    //     name: String::new(),
    //     email: String::new(),
    //     created_at: String::new(),
    // };

    html! {

        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <RustaceanForm rustacean={None}/>
                </div>
            </div>
        </div>
    }
}
