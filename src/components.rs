pub mod button;
pub mod color_control;
pub mod default_sidebar;
pub mod header;
pub mod icons;
pub mod pages;
pub mod sidebar;

use yew::prelude::*;

use crate::theme::Theme;

#[derive(Clone, PartialEq, Properties)]
pub struct TitleBoxProps {
    pub title: String,

    #[prop_or(None)]
    pub subtitle: Option<String>,
}

#[function_component]
pub fn TitleBox(props: &TitleBoxProps) -> Html {
    html! {
        <>
            <div class={ classes!("title-container") }>
                <h1> { props.title.clone() } </h1>
                if props.subtitle.is_some() {
                    <h3> { props.subtitle.clone().unwrap() } </h3>
                }
            </div>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SectionTitleProps {
    pub title: String,

    #[prop_or(None)]
    pub subtitle: Option<String>,
}

#[function_component]
pub fn SectionTitle(props: &SectionTitleProps) -> Html {
    html! {
        <>
            <div class={ classes!("section-title-container") }>
                <h2> { props.title.clone() } </h2>
                if props.subtitle.is_some() {
                    <h4> { props.subtitle.clone().unwrap() } </h4>
                }
            </div>
        </>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SectionProps {
    pub content: Html,
}

#[function_component]
pub fn Section(props: &SectionProps) -> Html {
    let theme = (*(use_context::<UseStateHandle<Theme>>().unwrap())).clone();
    html! {
        <div class={ classes!("section", theme.get_crt_overlay()) }>
            { props.content.clone() }
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub enum IconType {
    NerdFont(String),
    Inline(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    pub icon: IconType,

    #[prop_or_default]
    pub classes: Classes,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    match &props.icon {
        IconType::NerdFont(x) => {
            html! { <i class={ classes!("nf ".to_owned() + x + " fa-2x icon", props.classes.clone())}></i> }
        }
        IconType::Inline(x) => Html::from_html_unchecked(AttrValue::from(x.to_owned())),
    }
}
