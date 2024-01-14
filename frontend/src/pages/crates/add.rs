use crate::components::{header::Header, sidebar::Sidebar};
use crate::components::crate_form::CrateForm;
use yew::prelude::*;

#[function_component(CrateAdd)]
pub fn crate_add() -> Html {
    html! {

        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <CrateForm a_crate={None}/>
                </div>
            </div>
        </div>
    }
}
