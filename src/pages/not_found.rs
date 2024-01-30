use yew::prelude::*;
use yew_router::prelude::*;

use crate::{route::Route, pages};

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <Link<Route> to={Route::Home} classes={ "content-button" }>
                <h3>{ "return home" }</h3>
            </Link<Route>>
        </div>
    };
    pages::build_page(Some("404".to_string()), None, content)
}