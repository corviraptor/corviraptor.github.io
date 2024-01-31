use yew::prelude::*;

use crate::components::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,

    #[prop_or(ButtonAction::None)]
    pub action: ButtonAction,

    pub style: ButtonStyle,

    #[prop_or(None)]
    pub icon: Option<IconType>,
}

#[derive(Clone, PartialEq)]
pub enum ButtonAction {
    Url(String),
    StateChange(Callback<MouseEvent>),
    None,
}

#[derive(Clone, PartialEq)]
pub enum ButtonStyle {
    Content(Option<IconType>),
    Icon(IconType),
}

// this is nasty in terms of indentation i hate html
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let mut classes = match &props.style {
        ButtonStyle::Content(_) => "content-button".to_string(),
        ButtonStyle::Icon(_) => "icon-button".to_string(),
    };
    let mut icon_class = "icon".to_string();

    if props.action == ButtonAction::None {
        classes = classes.to_owned() + " " + &classes + "-disabled";
        icon_class = icon_class.to_owned() + " " + &icon_class + "-disabled";
    }

    let display = match &props.style {
        ButtonStyle::Content(x) => {
            html! {
                <>
                    <h3>{ props.name.clone() }</h3>

                    if let Some(y) = x {
                        <div class={ icon_class }>
                            <Icon icon={ y.clone() }/>
                        </div>
                    }
                </>
            }
        }
        ButtonStyle::Icon(x) => {
            html! {
                <div class={ icon_class }>
                    <Icon icon={ x.clone() }/>
                </div>
            }
        }
    };

    match &props.action {
        ButtonAction::None => html! {
            html! {
                <button disabled=true>
                    <div class={ classes }>
                        { display }
                    </div>
                </button>
            }
        },
        ButtonAction::Url(x) => html! {
            <a href={ x.clone() } target="_blank" rel="noopener noreferrer" class={ classes }>
                { display }
            </a>
        },
        ButtonAction::StateChange(x) => {
            let onclick = x.clone();
            html! {
                <button {onclick}>
                    <div class={ classes }>
                        { display }
                    </div>
                </button>
            }
        }
    }
}
