use yew::prelude::*;

use crate::{components::button::*, components::icons, components::*, pages};

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <SectionTitle title={"// stuff"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>

                    <LinkButton name={ "github" } url={ "https://github.com/corviraptor" } icon={ IconType::ForkAwesome("fa fa-github".to_string()) } />

                    <LinkButton name={ "bandcamp" } url={ "https://corviraptor.bandcamp.com" } icon={ IconType::ForkAwesome("fa fa-bandcamp".to_string()) } />

                    <LinkButton name={ "twitch" } url={ "https://www.twitch.tv/corviraptor" } icon={ IconType::ForkAwesome("fa fa-twitch".to_string()) } />

                    <LinkButton name={ "youtube" } url={ "https://www.twitch.tv/corviraptor" } icon={ IconType::ForkAwesome("fa fa-youtube-play".to_string()) } />

                    <LinkButton name={ "ko-fi" } url={ "https://ko-fi.com/corviraptor" } icon={ IconType::Inline(icons::KOFI.to_string()) } />

                </div>
            }}/>
            <SectionTitle title={"// socials"} subtitle={"i barely use social media these days, but here you go"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>

                    <LinkButton name={ "cohost" } url={ "https://cohost.org/corviraptor" } icon={ IconType::Inline(icons::COHOST.to_string())} />

                    <LinkButton name={ "linkedin" } url={ "https://www.linkedin.com/in/katy-winter/" } icon={ IconType::ForkAwesome("fa fa-linkedin-square".to_string()) } />

                    <LinkButton name={ "mastodon" } url={ "https://mastodon.gamedev.place/@corviraptor" } icon={ IconType::ForkAwesome("fa fa-mastodon".to_string()) } />

                    <LinkButton name={ "bsky" } url={ "https://bsky.app/profile/corviraptor.bsky.social" } icon={ IconType::Inline(icons::BSKY.to_string()) } />

                    <LinkButton name={ "twitter" } url={ "https://twitter.com/corviraptor" } icon={ IconType::ForkAwesome("fa fa-twitter".to_string()) } />

                </div>
            }}/>
        </div>
    };
    pages::build_page(Some("links".to_string()), None, content)
}
