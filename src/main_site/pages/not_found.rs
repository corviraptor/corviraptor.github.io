use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_site::{pages, route::Route};

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
