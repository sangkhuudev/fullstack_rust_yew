use crate::{components::{header::Header, sidebar::Sidebar, crate_list::CrateList}, context::CurrentUserContext, pages::navigator::Route};
use yew::prelude::*;
use yew_router::components::Redirect;

#[function_component(Crates)]
pub fn crates() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    match &current_user_ctx.token {
        Some(token) => {
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
                                <CrateList token={token.clone()} />
                            </Suspense>
                        </div>
                    </div>
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login} />
        }
    
    }
    
}
