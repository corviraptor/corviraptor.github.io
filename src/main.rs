mod route;
mod header;

use yew::prelude::*;

use route::Route;
use header::Header;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header></Header>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

