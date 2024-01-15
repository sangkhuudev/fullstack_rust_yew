use crate::{components::{header::Header, sidebar::Sidebar, crate_form::CrateForm}, context::CurrentUserContext, pages::navigator::Route, hooks::{use_rustacean, use_rustaceans}};
use yew::prelude::*;
use yew_router::components::Redirect;


#[function_component(CratesAdd)]
pub fn crates_add() -> Html {
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
                                <CrateAddForm 
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
pub struct CrateAddFormProps {
    pub token: AttrValue,
}

#[function_component(CrateAddForm)]
fn crate_add_form(props: &CrateAddFormProps) -> HtmlResult {
    let rustaceans = use_rustaceans(&props.token)?;

    Ok(html! {
        <CrateForm a_crate={None} authors={rustaceans} />
    })

}
