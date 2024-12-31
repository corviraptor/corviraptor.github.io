pub mod main_site;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_site::{header::*, nav::*, pages, route::Route};
use corviraptor_dot_dev::{components::default_sidebar::RightSidebar, theme::Theme};

// If you're wondering why I'm doing it this way instead of using a workspace to separate the library and the main site binary crates, it's just the easy way to get this working for Github Pages.

#[function_component]
fn App() -> Html {
    let state = use_state(Theme::from_storage);
    let theme = state.get_theme_string();
    html! {
        <HashRouter>
        <ContextProvider<UseStateHandle<Theme>> context={ state } >
        <div class={ "page-wrapper" } style={ theme }>

            <div class={ "side" }/>

            <div class={ "main-outer" }>
                <div class={ "main" }>

                    <Header/>
                    <Nav/>

                    <Switch<Route> render={ pages::page_content_switch } />

                </div>
            </div>

            <div class={ "side" }>
                <RightSidebar />
            </div>

        </div>
        </ContextProvider<UseStateHandle<Theme>>>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
