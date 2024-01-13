use crate::{components::{header::Header, sidebar::Sidebar, rustacean_form::RustaceanForm}, context::CurrentUserContext, pages::navigator::Route, hooks::use_rustacean};
use yew::prelude::*;
use yew_router::components::Redirect;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub rustacean_id: i32,
}

#[function_component(RustaceansEdit)]
pub fn rustaceans_edit(props: &Props) -> Html {
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
                                <RustaceansEditForm 
                                    rustacean_id={props.rustacean_id} 
                                    token={token.clone()}
                                />
                            </Suspense>
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
