use yew::prelude::*;

use crate::components::*;
use crate::components::icons;



#[function_component]
pub fn Title () -> Html {
    html! {
        <TitleBox title={"// links"} />
    }
}

#[function_component]
pub fn Content () -> Html {
    html! {
        <div>
            <SectionTitle title={"// stuff"} />
            <div class={ "section" }>
                <div class={ "content-button-wrapper" }>
                    <LinkButton name={ "github" } url={ "hhttps://github.com/corviraptor" } logo={ IconType::ForkAwesome("fa fa-github".to_string())  } />

                    <LinkButton name={ "bandcamp" } url={ "https://corviraptor.bandcamp.com" } logo={ IconType::ForkAwesome("fa fa-bandcamp".to_string()) } />

                    <LinkButton name={ "twitch" } url={ "https://www.twitch.tv/corviraptor" } logo={ IconType::ForkAwesome("fa fa-twitch".to_string())  } />

                    <LinkButton name={ "youtube" } url={ "https://www.twitch.tv/corviraptor" } logo={ IconType::ForkAwesome("fa fa-youtube-play".to_string())  } />

                    <LinkButton name={ "ko-fi" } url={ "https://ko-fi.com/corviraptor" } logo={  IconType::Inline(icons::KOFI.to_string()) } />
                </div>
            </div>
            <SectionTitle title={"// socials"} subtitle={"i barely use social media these days, but here i am!"} />
            <div class={ "section" }>
                <div class={ "content-button-wrapper" }>
                    <LinkButton name={ "cohost" } url={ "https://cohost.org/corviraptor" } logo={ IconType::Inline(icons::COHOST.to_string())} />

                    <LinkButton name={ "linkedin" } url={ "https://www.linkedin.com/in/katy-winter/" } logo={ IconType::ForkAwesome("fa fa-linkedin-square".to_string())  } />

                    <LinkButton name={ "twitter" } url={ "https://twitter.com/corviraptor" } logo={ IconType::ForkAwesome("fa fa-twitter".to_string())  } />
                </div>

            </div>
        </div>
    }
}


#[derive(Clone, PartialEq, Properties)]
pub struct LinkProps {
    pub name: String,

    pub url: String,

    #[prop_or(None)]
    pub logo: Option<IconType>,
}

#[function_component]
pub fn LinkButton(props: &LinkProps) -> Html {
    html! {      
        <a href={ props.url.clone() } target="_blank" rel="noopener noreferrer" class={ "content-button" }>
            <h3>{ props.name.clone() }</h3>

            if props.logo.is_some() {
                <div style={ "color: var(--main);" }>
                    <Icon logo={ props.logo.clone().unwrap() }/>
                </div>
            }
        </a>
    }
}