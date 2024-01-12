use yew::prelude::*;

pub struct HashRouter { 
    
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <p>{ "This website isn't finished! ☹ " }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

