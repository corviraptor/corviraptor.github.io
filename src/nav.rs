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
    let current_route = use_route::<Route>().unwrap(); // TODO: replace unwrap()

    let should_disable_button = tab_props.route == current_route;

    if should_disable_button {
        return html! { 
            <div class={ "tab tab-disabled" }>
                <h3>{ tab_props.clone().name }</h3>
            </div>
        }
    } else {
        return html! { 
            <a href={ tab_props.route.to_path() } class={ "tab tab-enabled" }>
                <h3>{ tab_props.clone().name }</h3>
            </a>
        }
    }
}
