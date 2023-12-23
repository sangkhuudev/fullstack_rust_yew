use yew::{prelude::*, platform::spawn_local};
use yew_router::prelude::*;

use crate::{context::CurrentUserContext, pages::navigator::Route, api::rustaceans::api_rustaceans};

#[function_component(RustaceanList)]
pub fn rustacean_list() -> Html {
    let current_user_ctx = use_context::<CurrentUserContext>()
        .expect("Current user context is missing");

    let rustaceans_handle = use_state(|| vec![]);
    let rustaceans = (*rustaceans_handle).clone();
    match &current_user_ctx.token {
        Some(token) => {
            let cloned_token = token.clone();
            spawn_local(async move {
                let response = api_rustaceans(&cloned_token).await.unwrap();
                rustaceans_handle.set(response);
            });
            html! {
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
                
            }

        },
        None => html! {
            <Redirect<Route> to={Route::Login} />
        }
    }
}
