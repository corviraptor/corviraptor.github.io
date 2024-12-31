use yew::prelude::*;

use crate::components::default_sidebar::RightSidebar;
use crate::components::header::Header;
use crate::components::*;

pub struct PageBuilder {
    pub title: String,
    pub subtitle: Option<String>,
    pub content: Html,
    pub theme: UseStateHandle<Theme>,
    pub left_sidebar: Option<Html>,
}

impl PageBuilder {
    pub fn new(title: String, theme: UseStateHandle<Theme>, content: Html) -> Self {
        Self {
            title,
            subtitle: None,
            content,
            theme,
            left_sidebar: None,
        }
    }

    pub fn with_left_sidebar(mut self, sidebar: Html) -> Self {
        self.left_sidebar = Some(sidebar);
        self
    }

    pub fn with_subtitle(mut self, subtitle: String) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn build(self) -> Html {
        let theme = self.theme.get_theme_string();

        html! {
            <ContextProvider<UseStateHandle<Theme>> context={ self.theme } >
            <div class={ "page-wrapper" } style={ theme }>

                <div class={ "side" }>
                    { self.left_sidebar }
                </div>

                <div class={ "main-outer" }>
                    <div class={ "main" }>

                        <Header/>
                        <PageContent title={self.title} subtitle={self.subtitle} content={self.content}/>

                    </div>
                </div>

                <div class={ "side" }>
                    <RightSidebar />
                </div>

            </div>
            </ContextProvider<UseStateHandle<Theme>>>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageContentProps {
    pub title: String,

    #[prop_or(None)]
    pub subtitle: Option<String>,

    pub content: Html,
}

#[function_component]
pub fn PageContent(props: &PageContentProps) -> Html {
    html! {
        <>
            <TitleBox title={"// ".to_string() + &props.title.to_lowercase()} subtitle={ props.subtitle.clone() }/>
            <div class={ "content" }>
                { props.content.clone() }
            </div>
        </>
    }
}
