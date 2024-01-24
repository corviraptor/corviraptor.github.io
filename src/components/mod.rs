use yew::prelude::*;

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
}

#[function_component]
pub fn SectionTitle(props: &SectionTitleProps) -> Html {
    html! {
        <>
            <div class={ "section-title-container" }>
                <h2> { props.title.clone() } </h2>
            </div>
        </>
    }
}