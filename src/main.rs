mod components;
mod header;
mod nav;
mod pages;
mod route;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{header::*, nav::*, route::Route};

#[function_component]
fn App() -> Html {
    html! {
        <HashRouter>
            <div class={ "page-wrapper" }>
                <div class={ "main-outer" }>
                    <div class={ "main" }>

                        <Header/>
                        <Nav/>

                        <Switch<Route> render={ pages::page_content_switch } />

                    </div>
                </div>
            </div>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
