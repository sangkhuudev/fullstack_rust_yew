use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: AttrValue,
    pub options: Vec<(AttrValue, AttrValue)>,
    pub name: AttrValue,
    pub value: AttrValue,
    pub onchange: Callback<Event>,
}
#[function_component(Select)]
pub fn select(props: &Props) -> Html {
    let html_id = format!("edit-{}", props.name);
    html! {
        <>
            <label for={html_id.clone()}>{props.label.clone()}</label>
            <select 
                class="form-control"
                id={html_id}
                name={props.name.clone()}
                onchange={props.onchange.clone()}
            >
                {
                    props.options.clone().into_iter().map(|option| {
                        html! {
                            <option value={option.0}>{option.1}</option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </>
    }
}
