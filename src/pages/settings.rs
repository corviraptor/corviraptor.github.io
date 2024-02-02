use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::{
    components::button::*,
    components::*,
    theme::{StyleType, Theme},
};

#[derive(Clone, PartialEq, Properties)]
pub struct SettingsProps {
    pub title: String,

    #[prop_or(None)]
    pub subtitle: Option<String>,
}

#[function_component]
pub fn Page() -> Html {
    html! {
        <>
            <CrtControl/>
            <FontControl/>


            <ColorControl style_type={ StyleType::Main } name={ "main" } />
            <ColorControl style_type={ StyleType::MainDark } name={ "main-dark" } />
            <ColorControl style_type={ StyleType::MainDarker } name={ "main-darker" } />
            <ColorControl style_type={ StyleType::MainHighlighted } name={ "main-highlighted" } />
            <ColorControl style_type={ StyleType::MainBright } name={ "main-bright" } />
            <ColorControl style_type={ StyleType::Accent } name={ "accent" } />
            <ColorControl style_type={ StyleType::AccentHighlighted } name={ "accent-highlighted" } />
            <ColorControl style_type={ StyleType::ButtonDisabled } name={ "button-disabled" } />
            <ColorControl style_type={ StyleType::Background } name={ "background" } />
            <ColorControl style_type={ StyleType::TextColor } name={ "text-color" } />
        </>
    }
}

#[function_component(CrtControl)]
pub fn crt_control() -> Html {
    let onclick = {
        let state = use_context::<UseStateHandle<Theme>>().unwrap();

        Callback::from(move |_| {
            let theme = (*state).clone();
            let crt_status = theme.crt_active;
            state.set(Theme {
                crt_active: !crt_status,
                ..theme
            })
        })
    };

    let action = ButtonAction::StateChange(onclick);

    html! {
        <>
            <IconButton name={ "Disable CRT Effect" } action={ action } icon={ IconType::ForkAwesome("fa fa-info-circle".to_string())  } />
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ColorControlProps {
    pub name: String,
    pub style_type: StyleType,
}

#[function_component(ColorControl)]
pub fn color_control(props: &ColorControlProps) -> Html {
    let input_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();
        let props = props.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();
            let theme = (*state).clone();

            if let Some(x) = input {
                state.set(theme.with_style(props.clone().style_type, x.value()));
            }
        })
    };

    let displayed_color = {
        if let Some(x) = theme.get_style(props.clone().style_type) {
            x.clone().value
        } else {
            "#ff00ff".to_string()
        }
    };

    html! {
        <>
            <label for={ "color picker" }>{ props.clone().name + " " + &displayed_color.clone() }</label>
            <input ref={ input_node_ref } { oninput } type={ "color" } id={ "color picker" } name={ "color picker" } value={ displayed_color.clone() }/>
        </>
    }
}

#[function_component(FontControl)]
pub fn font_control() -> Html {
    let select_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let onselect = {
        let select_node_ref = select_node_ref.clone();

        Callback::from(move |_| {
            let select = select_node_ref.cast::<HtmlSelectElement>();
            let theme = (*state).clone();

            if let Some(x) = select {
                state.set(theme.with_style(StyleType::Font, x.value()));
            } else {
                state.set(theme.with_style(StyleType::Font, "Wingdings".to_string()));
            }
        })
    };

    let displayed_font = {
        if let Some(x) = theme.get_style(StyleType::Font) {
            x.clone().value
        } else {
            "Wingdings".to_string()
        }
    };

    html! {
        <>
            <label for={ "font picker" }>{ {"Font: ".to_string()} + &displayed_font.clone() }</label>
            <select ref={ select_node_ref } name={ "font picker" } id={ "font picker" } { onselect } >
              <optgroup label={ "Slab Serif" }>
                <option value={ "'Iosevka Corax Web', monospace" } selected={ true }>{ "Iosevka Corax" }</option>
              </optgroup>
              <hr />
              <optgroup label={ "Serif" }>
                <option value={ "'Source Serif 4 Web', serif" }>{ "Source Serif 4" }</option>
                <option value={ "serif" }>{ "Default Serif" }</option>
              </optgroup>
              <optgroup label={ "Sans-Serif" }>
              <option value={ "'Iosevka Web', sans-serif" }>{ "Iosevka" }</option>
                <option value={ "'Atkinson Hyperlegible Web', sans-serif" }>{ "Atkinson Hyperlegible" }</option>
                <option value={ "sans-serif" }>{ "Default Sans-Serif" }</option>
              </optgroup>
            </select>
        </>
    }
}
