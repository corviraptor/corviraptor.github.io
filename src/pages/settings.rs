use palette::Srgb;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::theme::*;

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
            <h3> { "Style" } </h3>
            <CrtControl/>
            <FontControl/>
            <ThemeControl/>

            <h3> { "Colors" } </h3>
            <ColorControl color={ ColorType::Main } />
            <ColorControl color={ ColorType::Accent } />
            <ColorControl color={ ColorType::Highlight } />
            <ColorControl color={ ColorType::Disabled } />
            <ColorControl color={ ColorType::TextColor } />
            <ColorControl color={ ColorType::TextHighlight } />
        </>
    }
}

#[function_component(CrtControl)]
pub fn crt_control() -> Html {
    let input_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();
            let theme = (*state).clone();

            let value = if let Some(x) = input {
                x.value().contains("false")
            } else {
                false
            };

            state.set(Theme {
                crt_active: value,
                ..theme
            })
        })
    };

    html! {
        <div class={ "setting" }>
            <input ref={ input_node_ref } { oninput } type={ "checkbox" } id={ "crt_checkbox" } name={ "crt_checkbox" } value={ theme.clone().crt_active.to_string() }/>
            <label for={ "crt_checkbox" } class={"setting-label"}>{ "Disable CRT Effect" }</label>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ColorControlProps {
    pub color: ColorType,
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
                let color_text = x
                    .value()
                    .parse()
                    .unwrap_or(Srgb::new(255, 0, 255))
                    .into_format()
                    .into();
                state.set(theme.with_color(&SiteColor(props.color.clone(), color_text)));
            }
        })
    };

    let displayed_color: String = {
        let color = {
            if let Some(x) = theme.custom_colors.iter().find(|x| x.0 == props.color) {
                x.clone()
            } else {
                theme.color_theme.default_color(props.clone().color)
            }
        };
        let x: Srgb<u8> = color.1.color.into_format();
        format!("#{x:x}")
    };

    let color_id = props.color.get_id();

    html! {
        <div class={ "setting" }>
            <input ref={ input_node_ref } { oninput } type={ "color" } id={ color_id.clone() } name={ color_id.clone() } value={ displayed_color.clone() }/>
            <label for={ color_id.clone() } class={"setting-label"}>{ color_id.clone() + " (" + &displayed_color.clone() + ")" }</label>
        </div>
    }
}

#[function_component(FontControl)]
pub fn font_control() -> Html {
    let select_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let select_node_ref = select_node_ref.clone();

        Callback::from(move |_| {
            let select = select_node_ref.cast::<HtmlSelectElement>();
            let theme = (*state).clone();

            if let Some(x) = select {
                state.set(theme.with_font(SiteFont::from_str(&x.value())));
            }
        })
    };

    let displayed_font = theme.font.to_string();

    html! {
        <div class={ "setting" }>
            <select ref={ select_node_ref } name={ "font picker" } id={ "font picker" } { oninput } >
              <optgroup label={ "Monospace" }>
                <option value={ "Iosevka Corax" } selected={ true }>{ "Iosevka Corax" }</option>
                <option value={ "Iosevka" }>{ "Iosevka" }</option>
                <option value={ "Monospace" }>{ "Default Monospace" }</option>
              </optgroup>
              <hr />
              <optgroup label={ "Serif" }>
                <option value={ "Source Serif 4" }>{ "Source Serif 4" }</option>
                <option value={ "Serif" }>{ "Default Serif" }</option>
              </optgroup>
              <optgroup label={ "Sans-Serif" }>
                <option value={ "Atkinson Hyperlegible" }>{ "Atkinson Hyperlegible" }</option>
                <option value={ "Sans Serif" }>{ "Default Sans-Serif" }</option>
              </optgroup>
            </select>
            <label for={ "font picker" } class={"setting-label"}>{ {"Font: ".to_string()} + &displayed_font.clone() }</label>
        </div>
    }
}

#[function_component(ThemeControl)]
pub fn theme_control() -> Html {
    let input_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let select = input_node_ref.cast::<HtmlInputElement>();
            let theme = (*state).clone();

            if let Some(x) = select {
                state.set(theme.with_color_theme(ColorTheme::from_str(&x.value())));
            }
        })
    };

    let display = theme.color_theme.to_string();

    html! {
        <div class={ "setting" }>
            <select ref={ input_node_ref } name={ "theme picker" } id={ "theme picker" } { oninput } >
                <option value={ "Light" } selected={ theme.color_theme == ColorTheme::Light }>{ "Light" }</option>
                <option value={ "Dark" } selected={ theme.color_theme == ColorTheme::Dark }>{ "Dark" }</option>
            </select>
            <label for={ "font picker" } class={"setting-label"}>{ {"Color Theme: ".to_string() + &display }}</label>
        </div>
    }
}
