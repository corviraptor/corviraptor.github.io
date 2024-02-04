mod components;
mod header;
mod markdown;
mod nav;
mod pages;
mod route;
mod sidebar;
mod theme;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{header::*, nav::*, route::Route, sidebar::*, theme::Theme};

#[function_component]
fn App() -> Html {
    let state = use_state(Theme::new);
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
                <Sidebar />
            </div>

        </div>
        </ContextProvider<UseStateHandle<Theme>>>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
