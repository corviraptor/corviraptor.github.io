use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p>{ "Hello from App()!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

