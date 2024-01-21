mod route;
mod header;
mod nav;

use yew::prelude::*;
use yew_router::prelude::*;

use route::Route;
use header::Header;
use nav::Nav;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
        <div class={ "page-wrapper" }>
            <div class={ "main-outer" }>
                <div class={ "main" }>

                    <Header/>
                    <Nav/>

                </div>
            </div>
        </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

