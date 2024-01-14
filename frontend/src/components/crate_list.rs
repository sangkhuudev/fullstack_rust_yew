use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_crates;
use crate::pages::navigator::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token: AttrValue,
}

#[function_component(CrateList)]
pub fn crate_list(props: &Props) -> HtmlResult {
    let crates = use_crates(&props.token)?;

    Ok(html! {
        <>
            <p>
                <Link<Route> to={Route::CratesAdd}>
                    {"+ Add a new crate"}
                </Link<Route>>
            </p>

            <table class="table">
            <thead>
                <th>{"ID"}</th>
                <th>{"Code"}</th>
                <th>{"Name"}</th>
                <th>{"Rustacean Id"}</th>
                <th>{"Version"}</th>
                <th>{"Description"}</th>
                <th>{"Created at"}</th>
                <th>{"Operations"}</th>
            </thead>
            <tbody>
                {
                    crates.into_iter().map(|a_crate| {
                        html! {
                            <tr>
                                <td>{a_crate.id}</td>
                                <td>{a_crate.code}</td>
                                <td>{a_crate.name}</td>
                                <td>{a_crate.rustacean_id}</td>
                                <td>{a_crate.version}</td>
                                <td>{a_crate.description}</td>
                                <td>{a_crate.created_at}</td>
                                <td>
                                    <Link<Route> 
                                        to={Route::CratesEdit { id: a_crate.id }} 
                                        classes="link-secondary"
                                    >
                                        {"Edit"}
                                    </Link<Route>>
                                    <span class="mx-1">{"/"}</span>
                                    <Link<Route> to={Route::CratesDelete { id: a_crate.id}} classes="link-danger">
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
