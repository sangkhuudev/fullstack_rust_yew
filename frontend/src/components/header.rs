use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::{CurrentUserContext, CurrentUserActions, CurrentUserDispatcherAction};
use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");
    
    match &current_user_ctx.user {
        Some(user) => {
            let cloned_user_ctx = current_user_ctx.clone();
            let onclick = Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                cloned_user_ctx.dispatch(CurrentUserDispatcherAction {
                    action_type: CurrentUserActions::LoginFailure,
                    login_response: None,
                    me_response: None,

                });
            });
            html! {
                <div class="text-end">
                    <p>
                        <span class="pe-1">{"Welcome "}{&user.username}</span>
                        <button class="btn btn-danger" onclick={onclick}>{"Logout"}</button>
                    </p>
                </div>
            }
        },
        None => html! {
            <Redirect<Route> to={Route::Login} />
        }
    }

}
