use crate::components::{header::Header, sidebar::Sidebar, rustacean_list::RustaceanList};
use yew::prelude::*;

#[function_component(Rustaceans)]
pub fn rustaceans() -> Html {

    let loading = html! { <p>{"Loading ......"}</p>};
    html! {
        <div class="container">
            <div class="row">
                <div class="col-sm-auto">
                    <Sidebar />
                </div>
                <div class="col mt-3">
                    <Header />
                    <Suspense fallback={loading}> 
                        <RustaceanList />
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
