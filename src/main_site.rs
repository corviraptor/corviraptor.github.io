pub mod header;
pub mod nav;
pub mod pages;
pub mod route;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::main_site::{header::*, nav::*, route::Route};
use corviraptor_dot_dev::components::default_sidebar::RightSidebar;
use corviraptor_dot_dev::theme::Theme;

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
