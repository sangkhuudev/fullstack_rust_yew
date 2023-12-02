use yew::{UseReducerHandle, Reducible};
use yew::prelude::*;
use crate::api::user::{User, MeResponse, LoginResponse};

pub type CurrentUserContext = UseReducerHandle<CurrentUser>;


#[derive(Default, PartialEq)]
pub struct CurrentUser {
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Reducible for CurrentUser {
    type Action = CurrentUserDispatcherAction;
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action.action_type {
            CurrentUserActions::LoginSuccess =>  {
                let login_resp = action.login_response.expect("Missing login response");
                let me_resp = action.me_response.expect("Missing me response");

                Self {
                    user: Some( User {
                        id: me_resp.id,
                        username: me_resp.username,
                        created_at: me_resp.created_at,
                    }),
                    token: Some(login_resp.token)
                }.into()
            },

            CurrentUserActions::LoginFailure => {
                Self {
                    user: None,
                    token: None,
                }.into()
            }
        }
    }
}

pub struct CurrentUserDispatcherAction {
    pub action_type: CurrentUserActions,
    pub login_response: Option<LoginResponse>,
    pub me_response: Option<MeResponse>

}

pub enum CurrentUserActions {
    LoginSuccess,
    LoginFailure,
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(CurrentUser::default);

    html! {
        <ContextProvider<CurrentUserContext> context={user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>

    }
}
