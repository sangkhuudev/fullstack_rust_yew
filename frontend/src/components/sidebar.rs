use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let current_route = use_route::<Route>().expect("No current route defined");
    let home_classes = {
        if current_route == Route::Home {
            classes!("active", "nav-link")
        } else {
            classes!("nav-link")
        }
    };
    
    let rustaceans_classes = {
        if current_route == Route::Rustaceans {
            classes!("active", "nav-link")
        } else {
            classes!("nav-link")
        }
    };

    let crates_classes = {
        if current_route == Route::Crates {
            classes!("active", "nav-link")
        } else {
            classes!("nav-link")
        }
    };
    html! {
        <nav class="navbar navbar-light">
            <ul class="nav navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_classes}>
                        {"Home"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Rustaceans} classes={rustaceans_classes}>
                        {"Rustaceans"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Crates} classes={crates_classes}>
                        {"Crates"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
