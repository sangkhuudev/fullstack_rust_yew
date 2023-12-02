use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::CurrentUserContext;
use crate::components::sidebar::Sidebar;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");
    
    match &current_user_ctx.user {
        Some(user) => {
            html! {

                <div class="container">
                    <div class="row">
                        <div class="col">
                            <Sidebar />
                        </div>
                        <div class="col">
                            <h1>{"Welcome "}{&user.username}</h1>
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
