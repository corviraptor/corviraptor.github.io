use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class={ "header-container" }>
            <a href="/">
                <div class={ "logo-container" }>
                    <div class={ "header-title" }>
                        <h2 style="display: table-caption;">{ "katy winter" }</h2>
                    </div>

                    <Switch<Route> render={ return_home_button } />

                </div>
            </a>
            <div class={ "contact-container" }>
                <h2>{ "katywinter@corviraptor.net" }</h2>
            </div>
        </header>
    }
}

pub fn return_home_button(route: Route) -> Html {
    match route {
        Route::Home =>  html! { },
        _ => {  // this is more indentation than i would like but its whatever
            html! {
                <div class={ "return-home" }>
                    <h4>{ "return home" }</h4>
                </div>
            } 
        }
    }
}