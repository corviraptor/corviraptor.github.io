use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SidebarProps {
    content: Html,
}

#[function_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class={ "sidebar-container" }>
            <div class={ "sidebar" }>
                { props.to_owned().content }
            </div>
        </div>
    }
}

pub fn build_sidebar(content: Html) -> Html {
    html! {
        <div class={ "sidebar-container" }>
            <div class={ "sidebar" }>
                { content }
            </div>
        </div>
    }
}
