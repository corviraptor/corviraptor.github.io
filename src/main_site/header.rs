use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_site::route::Route;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class={ "header-container" }>
            <div class={ "left-header-container" }>
            <Link<Route> to={Route::Home}>
                <div class={ "logo-container" } >
                    <div class={ "header-title" }>
                        <h2 style="display: table-caption;">{ "katy winter" }</h2>
                    </div>
                </div>
            </Link<Route>>

                <Switch<Route> render={ return_home_button } />
            </div>
            <div class={ "contact-container" }>
                // <h2>{ "kathrynne@corviraptor.net" }</h2> TODO: uncomment this when i actually own the domain and have the email set up
            </div>
        </header>
    }
}

pub fn return_home_button(route: Route) -> Html {
    match route {
        Route::Home => html! {},
        _ => {
            // this is more indentation than i would like but its whatever
            html! {
                <Link<Route> to={Route::Home}>
                    <div class={ "return-home" }>
                        <h4>{ "return home" }</h4>
                    </div>
                </Link<Route>>
            }
        }
    }
}
