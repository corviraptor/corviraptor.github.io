mod components;
mod header;
mod nav;
mod pages;
mod route;
mod sidebar;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{header::*, nav::*, route::Route, sidebar::*};

#[function_component]
fn App() -> Html {
    html! {
        <HashRouter>
            <div class={ "page-wrapper" }>
                <div class={ "side" }/>

                <div class={ "main-outer" }>
                    <div class={ "main" }>

                        <Header/>
                        <Nav/>

                        <Switch<Route> render={ pages::page_content_switch } />

                    </div>
                </div>

                <div class={ "side" }>
                    <Sidebar />
                </div>
            </div>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
