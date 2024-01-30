use yew::prelude::*;

use crate::{components::icons, components::*, pages};

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <SectionTitle title={"// stuff"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>
                    <ContentButton name={ "github" } url={ "https://github.com/corviraptor" } logo={ IconType::ForkAwesome("fa fa-github".to_string()) } />

                    <ContentButton name={ "bandcamp" } url={ "https://corviraptor.bandcamp.com" } logo={ IconType::ForkAwesome("fa fa-bandcamp".to_string()) } />

                    <ContentButton name={ "twitch" } url={ "https://www.twitch.tv/corviraptor" } logo={ IconType::ForkAwesome("fa fa-twitch".to_string()) } />

                    <ContentButton name={ "youtube" } url={ "https://www.twitch.tv/corviraptor" } logo={ IconType::ForkAwesome("fa fa-youtube-play".to_string()) } />

                    <ContentButton name={ "ko-fi" } url={ "https://ko-fi.com/corviraptor" } logo={ IconType::Inline(icons::KOFI.to_string()) } />
                </div>
            }}/>
            <SectionTitle title={"// socials"} subtitle={"i barely use social media these days, but here you go"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>
                    <ContentButton name={ "cohost" } url={ "https://cohost.org/corviraptor" } logo={ IconType::Inline(icons::COHOST.to_string()) } />

                    <ContentButton name={ "linkedin" } url={ "https://www.linkedin.com/in/katy-winter/" } logo={ IconType::ForkAwesome("fa fa-linkedin-square".to_string()) } />

                    <ContentButton name={ "mastodon" } url={ "https://mastodon.gamedev.place/@corviraptor" } logo={ IconType::ForkAwesome("fa fa-mastodon".to_string()) } />

                    <ContentButton name={ "bsky" } url={ "https://bsky.app/profile/corviraptor.bsky.social" } logo={ IconType::Inline(icons::BSKY.to_string()) } />

                    <ContentButton name={ "twitter" } url={ "https://twitter.com/corviraptor" } logo={ IconType::ForkAwesome("fa fa-twitter".to_string())  } />
                </div>
            }}/>
        </div>
    };
    pages::build_page(Some("links".to_string()), None, content)
}
