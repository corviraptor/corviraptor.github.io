use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::components::*;

#[function_component]
pub fn Title () -> Html {
    html! {
        <TitleBox title={"// 404 "} />
    }
}

#[function_component]
pub fn Content() -> Html {
    html! {
        <div class={ "section" }>
            <div class={ "content-button-wrapper" }>
                <Link<Route> to={Route::Home} classes={ "content-button" }>
                    <h3>{ "return home" }</h3>
                </Link<Route>>
            </div>
        </div>
    }
}