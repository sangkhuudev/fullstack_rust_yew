use web_sys::HtmlInputElement;
use yew::{prelude::*, platform::spawn_local};
use crate::{components::input::*, api::user::*, components::alert::*};
use gloo_console::log;


async fn login(username: String, password: String) -> Result<(LoginResponse,MeResponse), gloo_net::Error> {
    let login_response = api_login(username, password).await?;
    let me_response = api_me(&login_response.token).await?;

    Ok((login_response, me_response))
}
#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
    let password_handle = use_state(String::default);
    let password = (*password_handle).clone();
    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let username_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            username_handle.set(input.value())
        }
    });
    let password_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_handle.set(input.value())
        }
    });
    let cloned_username = username.clone();
    let cloned_password = password.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let cloned_username = cloned_username.clone();
        let cloned_password = cloned_password.clone();
        let cloned_error = error_message_handle.clone();
        spawn_local(async move {
            match login(cloned_username, cloned_password).await {
                Ok(response) => log!(response.1.username),
                Err(error) => cloned_error.set(error.to_string())
            }
        })
    });
    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert message={error_message} alert_type={"danger"} />
            }
            <div class="mb-3">
                <Input 
                    label="Username" 
                    input_type="text" 
                    name="username" 
                    value={username.clone()}
                    onchange={username_changed}
                />
            </div>
            <div class="mb-3">
                <Input 
                    label="Password" 
                    input_type="password" 
                    name="password" 
                    value={password}
                    onchange={password_changed}
                />
            </div>
            <button type="submit" class="btn btn-primary">{"Login"}</button>
        </form>
    }
}
