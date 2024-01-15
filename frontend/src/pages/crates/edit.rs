use crate::{
    components::{header::Header, sidebar::Sidebar, crate_form::CrateForm}, 
    context::CurrentUserContext, pages::navigator::Route, hooks::{use_crate, use_rustaceans},
};
use yew::prelude::*;
use yew_router::components::Redirect;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub crate_id: i32,
}

#[function_component(CratesEdit)]
pub fn crates_edit(props: &Props) -> Html {
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
                                <CratesEditForm 
                                    crate_id={props.crate_id} 
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
pub struct CratesEditFormProps {
    pub crate_id: i32,
    pub token: AttrValue,
}

#[function_component(CratesEditForm)]
fn crates_edit_form(props: &CratesEditFormProps) -> HtmlResult {
    let a_crate = use_crate(&props.token, props.crate_id.clone())?;
    let rustaceans = use_rustaceans(&props.token)?;
    Ok(html! {
        <CrateForm a_crate={a_crate} authors={rustaceans} />
    })

}
