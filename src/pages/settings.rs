use palette::Srgb;
use std::str::FromStr;
use std::string::ToString;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::components::button::*;
use crate::components::IconType;
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
            <TextThemeControl/>

            <h3> { "Colors" } </h3>
            <ColorControl color={ ColorDomain::Main(MainColor::Main) } />
            <ColorControl color={ ColorDomain::Main(MainColor::Accent) } />
            <ColorControl color={ ColorDomain::Main(MainColor::Highlight)} />
            <ColorControl color={ ColorDomain::Main(MainColor::Disabled) } />
            <ColorControl color={ ColorDomain::Text(ScreenColor::Body) } />
            <ColorControl color={ ColorDomain::Text(ScreenColor::Main) } />
            <ColorControl color={ ColorDomain::Text(ScreenColor::Highlight) } />
            <ColorControl color={ ColorDomain::Text(ScreenColor::Border) } />
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
    pub color: ColorDomain,
}

#[function_component(ColorControl)]
pub fn color_control(props: &ColorControlProps) -> Html {
    let input_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();
        let props = props.clone();
        let state = state.clone();

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
                state.set(theme.with_color(&SiteColor::new(props.color.clone(), color_text)));
            }
        })
    };

    let onclick = {
        let props = props.clone();
        let state = state.clone();

        Callback::from(move |_| {
            let theme = (*state).clone();
            state.set(theme.without_color(&props.color));
        })
    };

    let maybe_color = theme.custom_colors.iter().find(|x| x.domain == props.color);

    let color_changed: bool = maybe_color.is_some();

    let displayed_color: String = {
        let color = {
            if let Some(x) = maybe_color {
                // TODO: this sucks, fix this
                x.clone()
            } else {
                match props.clone().color {
                    ColorDomain::Main(x) => theme.color_theme.default_color(x),
                    ColorDomain::Text(x) => theme.text_theme.default_color(x),
                }
            }
        };
        let x: Srgb<u8> = color.value.color.into_format();
        format!("#{x:x}")
    };

    let color_id = props.color.get_string();

    let action = {
        if color_changed {
            ButtonAction::StateChange(onclick)
        } else {
            ButtonAction::None
        }
    };

    html! {
        <div class={ "setting" }>
            <IconButton name={ color_id.clone() } action={ action } icon={ IconType::Unicode('⭯') } style={ ButtonStyle::Screen } />
            <input ref={ input_node_ref } { oninput } type={ "color" } id={ color_id.clone() } name={ color_id.clone() } value={ displayed_color.clone() }/>
            <label for={ color_id.clone() }>{ color_id.clone() + " (" + &displayed_color.clone() + ")" }</label>
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
                state.set(
                    theme.with_font(SiteFont::from_str(&x.value()).unwrap_or(SiteFont::Default)),
                );
            }
        })
    };

    let displayed_font = theme.font.to_string();

    html! {
        <div class={ "setting" }>
            <select ref={ select_node_ref } name={ "font picker" } id={ "font picker" } { oninput } >
            <option value={ "System Default" }>{ "System Default" }</option>
                <option value={ "Iosevka Corax" } selected={ true }>{ "Iosevka Corax (Monospace)" }</option>
                <option value={ "Iosevka" }>{ "Iosevka (Monospace)" }</option>
                <option value={ "Source Serif 4" }>{ "Source Serif 4 (Serif)" }</option>
                <option value={ "Atkinson Hyperlegible" }>{ "Atkinson Hyperlegible (Sans-Serif)" }</option>
            </select>
            <label for={ "font picker" }>{ {"Font: ".to_string()} + &displayed_font.clone() }</label>
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
                state.set(theme.with_color_theme(
                    ColorTheme::from_str(&x.value()).unwrap_or(ColorTheme::Cherry),
                ));
            }
        })
    };

    let display = theme.color_theme.to_string();

    html! {
        <div class={ "setting" }>
            <select ref={ input_node_ref } name={ "theme picker" } id={ "theme picker" } { oninput } >
                <option value={ "cherry" } selected={ theme.color_theme == ColorTheme::Cherry }>{ "Cherry Red" }</option>
                <option value={ "steel" } selected={ theme.color_theme == ColorTheme::Steel }>{ "Steel (Dark)" }</option>
            </select>
            <label for={ "theme picker" }>{ {"Color Theme: ".to_string() + &display }}</label>
        </div>
    }
}

#[function_component(TextThemeControl)]
pub fn text_theme_control() -> Html {
    let input_node_ref = use_node_ref();

    let state = use_context::<UseStateHandle<Theme>>().unwrap();
    let theme = (*state).clone();

    let oninput = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let select = input_node_ref.cast::<HtmlInputElement>();
            let theme = (*state).clone();

            if let Some(x) = select {
                state.set(theme.with_text_theme(
                    TextTheme::from_str(&x.value()).unwrap_or(TextTheme::Terminal),
                ));
            }
        })
    };

    let display = theme.text_theme.to_string();

    html! {
        <div class={ "setting" }>
            <select ref={ input_node_ref } name={ "text theme picker" } id={ "text theme picker" } { oninput } >
                <option value={ "terminal" } selected={ theme.text_theme == TextTheme::Terminal }>{ "Terminal (Dark)" }</option>
                <option value={ "calculator" } selected={ theme.text_theme == TextTheme::Calculator }>{ "Calculator (Light)" }</option>
            </select>
            <label for={ "text theme picker" }>{ {"Text Theme: ".to_string() + &display }}</label>
        </div>
    }
}
