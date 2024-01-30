use yew::prelude::*;

use crate::components::*;

#[function_component]
pub fn Sidebar() -> Html {
    html! {
        <div class={ "sidebar-container" }>
            <div class={ "sidebar" }>
                <div class={ "sidebar-controls-outer" }>
                    <div class={ "sidebar-controls" }>
                        <IconButton name={ "accessibility" } url={ "https://ko-fi.com/corviraptor" } icon={ IconType::ForkAwesome("fa fa-universal-access".to_string()) } />
                        <IconButton name={ "menu" } url={ "https://ko-fi.com/corviraptor" } icon={ IconType::ForkAwesome("fa fa-bars".to_string()) } />
                        <IconButton name={ "settings" } url={ "https://ko-fi.com/corviraptor" } icon={ IconType::ForkAwesome("fa fa-cog".to_string()) } />
                    </div>
                </div>
                <div class={ "sidebar-content-outer" }>
                    <div class={ "sidebar-content" }>
                        { "WAdujgnisdohnmsdfjkhsd rfujnighsdrfmujikhnhkd jishndfWAdujgnisdohnmsdfjkhsdrfujnighsdrfmujikhnhk djishndfWAdujgnisdohnmsdfjkhsdrfujnighs drfmujikhnhkdjishndfWAdujgnisdohnmsdfjkhsdr fujnighsdrfmujikhnhkdjishndf" }
                    </div>
                </div>
            </div>
        </div>
    }
}
