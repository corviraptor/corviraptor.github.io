pub mod header;
pub mod nav;
pub mod pages;
pub mod route;
pub mod sidebar;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_site::{header::*, nav::*, route::Route, sidebar::*};
use corviraptor_dot_dev::theme::Theme;

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
