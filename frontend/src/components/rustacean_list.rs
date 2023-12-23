use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_rustaceans;
use crate::{
    context::CurrentUserContext, pages::navigator::Route,
};

#[function_component(RustaceanList)]
pub fn rustacean_list() -> HtmlResult {
    let current_user_ctx =
        use_context::<CurrentUserContext>().expect("Current user context is missing");
    let token = current_user_ctx.token.as_ref().expect("Token is missing");
    let rustaceans= use_rustaceans(&token)?;

    Ok(html! {
        <>
            <p>
                <Link<Route> to={Route::RustaceansAdd}>
                    {"+ Add a new rustacean"}
                </Link<Route>>
            </p>

            <table class="table">
            <thead>
                <th>{"ID"}</th>
                <th>{"Name"}</th>
                <th>{"Email"}</th>
                <th>{"Created at"}</th>
                <th>{"Operations"}</th>
            </thead>
            <tbody>
                {
                    rustaceans.into_iter().map(|rustacean| {
                        html! {
                            <tr>
                                <td>{rustacean.id}</td>
                                <td>{rustacean.name}</td>
                                <td>{rustacean.email}</td>
                                <td>{rustacean.created_at}</td>
                                <td>
                                    <Link<Route> to={Route::RustaceansAdd}>
                                        {"Edit"}
                                    </Link<Route>>
                                    <span>{"/"}</span>
                                    <Link<Route> to={Route::RustaceansAdd}>
                                        {"Delete"}
                                    </Link<Route>>
                                </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
        </>

    })
}
