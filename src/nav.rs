use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq, Clone)]
struct NavTabProps {
    route: Route,
    name: String
}

#[function_component]
pub fn Nav() -> Html {
    html! {
        <nav> 
            <NavTab route={Route::Portfolio} name={"portfolio"} />

            <NavTab route={Route::Faq} name={"FAQ"} />

            <NavTab route={Route::Links} name={"links"} />
        </nav>
    }
}

#[function_component]
fn NavTab(tab_props: &NavTabProps) -> Html {
    let current_route = use_route::<Route>();

    match current_route {
        Some(x) if x == tab_props.route => html! { 
            <div class={ "tab tab-disabled" }>
                <h3>{ tab_props.clone().name }</h3>
            </div>
        },
        _ => html! { 
            <Link<Route> to={tab_props.route.clone()} classes={ "tab tab-enabled" }>
                <h3>{ tab_props.clone().name }</h3>
            </Link<Route>>
        }
    }
}
