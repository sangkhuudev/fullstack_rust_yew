use crate::api::crates::{Crate, api_crate_update, api_crate_create};
use crate::api::rustaceans::Rustacean;
use crate::pages::navigator::Route;
use crate::{
    components::alert::*, components::input::*, context::*, 
    components::select::Select, components::textarea::TextArea
};
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;


#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub a_crate: Option<Crate>,
    pub authors: Vec<Rustacean>,
}

#[function_component(CrateForm)]
pub fn crate_form(props: &Props) -> Html {
    let navigator = use_navigator().expect("Navigator not available");
    let name_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.name.clone();
        }
        String::default()
    });
    let name = (*name_handle).clone();
    let version_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.version.clone();
        }
        String::default()
    });
    let version = (*version_handle).clone();
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");

    let code_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.code.clone();
        }
        String::default()
    });
    let code = (*code_handle).clone();
    let rustacean_id_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            return c.rustacean_id.to_string();
        }
        String::default()
    });
    let rustacean_id = (*rustacean_id_handle).clone();

    let description_handle = use_state(|| {
        if let Some(c) = &props.a_crate {
            if let Some(description) = &c.description {
                return description.clone();
            }
        }
        String::default()
    });
    let description = (*description_handle).clone();

    let error_message_handle = use_state(String::default);
    let error_message = (*error_message_handle).clone();

    let name_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_handle.set(input.value())
        }
    });
    let code_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            code_handle.set(input.value())
        }
    });
    let version_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            version_handle.set(input.value())
        }
    });

    let rustacean_id_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlSelectElement>();
        if let Some(input) = target {
            rustacean_id_handle.set(input.value())
        }
    });

    let description_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(input) = target {
            description_handle.set(input.value())
        }
    });

    let cloned_name = name.clone();
    let cloned_code = code.clone();
    let cloned_rustacean_id = rustacean_id.clone();
    let cloned_version = version.clone();
    let cloned_description = description.clone();

    let cloned_a_crate = props.a_crate.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let cloned_name = cloned_name.clone();
        let cloned_code = cloned_code.clone();
        let cloned_rustacean_id = cloned_rustacean_id.clone();
        let cloned_version = cloned_version.clone();
        let cloned_description = cloned_description.clone();

        let cloned_error_handle = error_message_handle.clone();
        let cloned_navigator = navigator.clone();
        let cloned_user_ctx = current_user_ctx.clone();
        let cloned_a_crate = cloned_a_crate.clone();
        match &cloned_user_ctx.token {
            Some(token) => {
                let cloned_token = token.clone();
                let parsed_rustacean_id = cloned_rustacean_id.parse::<i32>();
                match parsed_rustacean_id {
                    Ok(rustacean_id) => spawn_local(async move {
                        if let Some(a_crate) = cloned_a_crate {
                            match api_crate_update(
                                &cloned_token,
                                a_crate.id,
                                &cloned_name,
                                &cloned_code,
                                rustacean_id,
                                &cloned_version,
                                &cloned_description
                            )
                            .await {
                                Ok(_) => cloned_navigator.push(&Route::Crates),
                                Err(error) => cloned_error_handle.set(error.to_string()),
                            }
                        } else {
                            match api_crate_create(
                                &cloned_token, 
                                &cloned_name, 
                                &cloned_code,
                                rustacean_id,
                                &cloned_version,
                                &cloned_description
                            ).await
                            {
                                Ok(_) => cloned_navigator.push(&Route::Crates),
                                Err(error) => cloned_error_handle.set(error.to_string()),
                            }
                        }
                    }),
                    Err(_) =>  cloned_error_handle.set("Cannot parse rustacean ID".to_string())
                }
                
            }
            None => cloned_error_handle.set("Session expired. Please login again".to_string()),
        }
    });

    let options = props.authors
        .iter()
        .map(|r| (AttrValue::from(r.id.to_string()), AttrValue::from(r.name.clone())))
        .collect::<Vec<(AttrValue, AttrValue)>>();

    html! {
        <form onsubmit={onsubmit}>
            if error_message.len() > 0 {
                <Alert message={error_message} alert_type={"danger"} />
            }
            <div class="mb-3">
                <Input
                    label="Name"
                    input_type="text"
                    name="name"
                    value={name.clone()}
                    onchange={name_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    label="Code"
                    input_type="text"
                    name="code"
                    value={code}
                    onchange={code_changed}
                />
            </div>
            <div class="mb-3">
                <Input
                    label="Version"
                    input_type="text"
                    name="version"
                    value={version}
                    onchange={version_changed}
                />
            </div>
            <div class="mb-3">
                <Select
                    label="Author"
                    name="author"
                    value={rustacean_id}
                    onchange={rustacean_id_changed}
                    options={options}
                />
            </div>
            <div class="mb-3">
            <TextArea
                label="Description"
                name="description"
                value={description}
                onchange={description_changed}
            />
        </div>
            <button type="submit" class="btn btn-primary">{"Save"}</button>
        </form>
    }
}
