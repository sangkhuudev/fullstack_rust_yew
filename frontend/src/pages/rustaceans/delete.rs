use crate::{components::{header::Header, sidebar::Sidebar, rustacean_form::RustaceanForm, alert::Alert}, context::CurrentUserContext, pages::navigator::Route, hooks::use_rustacean, api::rustaceans::api_rustacean_delete};
use yew::{prelude::*, platform::spawn_local};
use yew_router::{components::Redirect, hooks::use_navigator};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean_id: i32,
}

#[function_component(RustaceansDelete)]
pub fn rustaceans_delete(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let current_user_ctx = use_context::<CurrentUserContext>()
    .expect("Current user context is missing");

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    match &current_user_ctx.token {
        Some(token) => {
            let cloned_id = props.rustacean_id.clone();
            let cloned_token = token.to_owned();
            let onclick = Callback::from( move |e: MouseEvent| {
                e.prevent_default();
                let cloned_navigator = navigator.clone();
                let cloned_error_message = error_message_handle.clone();
                let cloned_id = cloned_id.clone();
                let cloned_token = cloned_token.clone();
                spawn_local(async move {
                    match api_rustacean_delete(&cloned_token, cloned_id).await {
                        Ok(()) => cloned_navigator.push(&Route::Rustaceans),
                        Err(err) => cloned_error_message.set(err.to_string()),
                    }
                })
            });
            html! {
                <div class="container">
                    <div class="row">
                        <div class="col-sm-auto">
                            <Sidebar />
                        </div>
                        <div class="col mt-3">
                            <Header />
                            if error_message.len() > 0 {
                                <Alert message={error_message} alert_type={"danger"} />
                            }
                            <p>
                                {"Are you sure you want to delete rustacean #"}
                                {&props.rustacean_id} {"?"}
                            </p>
                            <button onclick={onclick} class="btn btn-danger">{"Delete Rustacean"}</button>
                        </div>
                    </div>
                </div>
            }
        }
        None => html! {
            <Redirect<Route> to={Route::Login} />
        }
    }
    
}



#[derive(Properties, PartialEq)]
pub struct RustaceansEditFormProps {
    pub rustacean_id: i32,
    pub token: AttrValue,
}

#[function_component(RustaceansEditForm)]
fn rustaceans_edit_form(props: &RustaceansEditFormProps) -> HtmlResult {
    let rustacean = use_rustacean(props.token.as_str(), props.rustacean_id.clone())?;

    Ok(html! {
        <RustaceanForm rustacean={rustacean} />
    })

}
