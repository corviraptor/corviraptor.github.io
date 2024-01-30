use yew::prelude::*;

pub mod icons;

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
            <div class={ "title-container" }>
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
            <div class={ "section-title-container" }>
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
    html! {
        <div class={ "section ca-text" }>
            <div class={ "crt" }/>
            { props.content.clone() }
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub enum IconType {
    ForkAwesome(String),
    Inline(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    pub icon: IconType,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    match &props.icon {
        IconType::ForkAwesome(x) => html! { <i class={ x.clone() + " fa-2x"}></i> },
        IconType::Inline(x) => Html::from_html_unchecked(AttrValue::from(x.to_owned())),
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContentButtonProps {
    pub name: String,

    pub url: String,

    #[prop_or(None)]
    pub icon: Option<IconType>,
}

#[function_component]
pub fn ContentButton(props: &ContentButtonProps) -> Html {
    html! {
        <a href={ props.url.clone() } target="_blank" rel="noopener noreferrer" class={ "content-button" }>
            <h3>{ props.name.clone() }</h3>

            if props.icon.is_some() {
                <div class={ "icon" }>
                    <Icon icon={ props.icon.clone().unwrap() }/>
                </div>
            }
        </a>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct IconButtonProps {
    pub name: String,

    pub url: String,

    #[prop_or(None)]
    pub icon: Option<IconType>,
}

#[function_component]
pub fn IconButton(props: &IconButtonProps) -> Html {
    html! {
        <a href={ props.url.clone() } target="_blank" rel="noopener noreferrer" class={ "icon-button" }>
            if props.icon.is_some() {
                <div class={ "icon" }>
                    <Icon icon={ props.icon.clone().unwrap() }/>
                </div>
            }
        </a>
    }
}
