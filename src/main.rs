mod route;
mod header;
mod nav;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use route::Route;
use header::Header;
use nav::Nav;

#[function_component]
fn App() -> Html {
    html! {
        <HashRouter>
        <div class={ "page-wrapper" }>
            <div class={ "main-outer" }>
                <div class={ "main" }>

                    <Header/>
                    <Nav/>

                    <Switch<Route> render={ pages::page_switch } />

                </div>
            </div>
        </div>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
