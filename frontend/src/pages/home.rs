use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <h1>{"Welcome user ..."}</h1>
        </div>
    }
}
