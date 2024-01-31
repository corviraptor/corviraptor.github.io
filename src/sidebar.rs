use yew::prelude::*;

use crate::components::button::*;
use crate::components::*;

#[derive(PartialEq, Clone)]
pub enum SidebarState {
    Menu,
    Settings,
    Info,
}

#[function_component]
pub fn Sidebar() -> Html {
    let state = use_state_eq(|| SidebarState::Menu);
    let content = match *state {
        SidebarState::Menu => "Menu!!!",
        SidebarState::Settings => "Settings!!!!!!!!",
        SidebarState::Info => "Info!!!!!!!!!!!!!!!!!!!",
    };

    html! {
        <div class={ "sidebar-container" }>
            <div class={ "sidebar" }>

                <div class={ "sidebar-controls-outer" }>
                    <div class={ "sidebar-controls" }>
                        <Control name={ "menu" } icon={ IconType::ForkAwesome("fa fa-bars".to_string()) } button_state={ SidebarState::Menu } current_state = { state.clone() }/>
                        <Control name={ "settings" } icon={ IconType::ForkAwesome("fa fa-cog".to_string()) } button_state={ SidebarState::Settings } current_state = { state.clone() }/>
                        <Control name={ "info" } icon={ IconType::ForkAwesome("fa fa-info-circle".to_string()) } button_state={ SidebarState::Info } current_state = { state.clone() }/>
                    </div>
                </div>

                <div class={ "sidebar-content-outer" }>
                    <div class={ "sidebar-content" }>
                        { content }
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ControlProps {
    name: String,
    icon: IconType,
    button_state: SidebarState,
    current_state: UseStateHandle<SidebarState>,
}

#[function_component]
pub fn Control(props: &ControlProps) -> Html {
    let our_props = props.clone();
    let onclick = {
        let state = our_props.current_state.clone();
        Callback::from(move |_| state.set(our_props.clone().button_state))
    };
    let action = {
        if props.button_state == *props.current_state {
            ButtonAction::None
        } else {
            ButtonAction::StateChange(onclick)
        }
    };
    html! {
        <Button name={ props.clone().name } action={ action } style={ ButtonStyle::Icon(props.clone().icon) } />
    }
}
