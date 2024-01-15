use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}
#[function_component(TextArea)]
pub fn textarea(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <>
            <label for={html_id.clone()}>{props.label.clone()}</label>
            <textarea 
                class="form-control"
                id={html_id}
                name={props.name.clone()}
                value={props.value.clone()}
                onchange={props.onchange.clone()}
            />
        </>
    }
}
