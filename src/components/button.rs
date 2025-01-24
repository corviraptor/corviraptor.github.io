use yew::prelude::*;

use crate::components::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,

    #[prop_or(ButtonAction::None)]
    pub action: ButtonAction,

    pub button_type: ButtonType,

    pub button_style: ButtonStyle,

    #[prop_or(classes!())]
    pub classes: Classes,
}

#[derive(Clone, PartialEq)]
pub enum ButtonAction {
    Url(String),
    StateChange(Callback<MouseEvent>),
    None,
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Content(Option<IconType>),
    Icon(IconType),
}

#[derive(Clone, PartialEq)]
pub enum ButtonStyle {
    Physical,
    Screen,
    Bare,
}

// this is nasty in terms of indentation i hate html
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let mut classes = classes!("button", "site-button");

    classes.push(classes!(match &props.button_type {
        ButtonType::Content(_) => "content-button".to_string(),
        ButtonType::Icon(_) => "icon-button".to_string(),
    }));

    classes.push(classes!(match &props.button_style {
        ButtonStyle::Physical => "physical-button",
        ButtonStyle::Screen => "screen-button",
        ButtonStyle::Bare => "bare-button",
    }));

    classes.push(props.classes.clone());

    let icon_class = "icon".to_string();

    let display = match &props.button_type {
        ButtonType::Content(icon) => {
            html! {
                <>
                    <h3>{ props.name.clone() }</h3>

                    if let Some(icon) = icon {
                        <div class={ icon_class }>
                            <Icon icon={ icon.clone() }/>
                        </div>
                    }
                </>
            }
        }
        ButtonType::Icon(icon) => {
            html! {
                <div class={ icon_class }>
                    <Icon icon={ icon.clone() }/>
                </div>
            }
        }
    };

    match &props.action {
        ButtonAction::None => html! {
            <button class={ classes } disabled=true>
                { display }
            </button>
        },
        ButtonAction::Url(x) => {
            html! {
                <a href={ x.clone() } target="_blank" rel="noopener noreferrer"  class={ classes }>
                    {display}
                </a>
            }
        }
        ButtonAction::StateChange(x) => {
            let onclick = x.clone();
            html! {
                <button { onclick } class={ classes }>
                    { display }
                </button>
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct LinkButtonProps {
    pub name: String,

    #[prop_or(None)]
    pub url: Option<String>,

    #[prop_or(None)]
    pub icon: Option<IconType>,

    #[prop_or(ButtonStyle::Screen)]
    pub style: ButtonStyle,

    #[prop_or(classes!())]
    pub classes: Classes,
}

#[function_component]
pub fn LinkButton(props: &LinkButtonProps) -> Html {
    let action = match &props.url {
        None => ButtonAction::None,
        Some(x) => ButtonAction::Url(x.clone()),
    };
    html! {
        <Button name={ props.name.clone() } action={ action } button_type={ ButtonType::Content(props.icon.clone()) } button_style={ props.style.clone() } classes={ props.classes.clone() } />
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonProps {
    pub name: String,

    #[prop_or(ButtonAction::None)]
    pub action: ButtonAction,

    pub style: ButtonStyle,

    pub icon: IconType,

    #[prop_or(classes!())]
    pub classes: Classes,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    html! {
        <Button name={ props.name.clone() } action={ props.action.clone() } button_type={ ButtonType::Icon(props.icon.clone()) } button_style={ props.style.clone() } classes={ props.classes.clone() } />
    }
}
