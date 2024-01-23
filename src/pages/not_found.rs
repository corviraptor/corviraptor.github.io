use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component]
pub fn Content() -> Html {
    html! {
        <>
        <div class={ "title-container" }>
            <h1> { "// 404 " } </h1>
        </div>
        <div class={ "section" }>
            <div class={ "content-button-wrapper" }>
            <Link<Route> to={Route::Home} classes={ "content-button" }>
                <h3>{ "return home" }</h3>
            </Link<Route>>
            </div>
        </div>
        </>
    }
}