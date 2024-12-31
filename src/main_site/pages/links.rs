use yew::prelude::*;

use corviraptor_dot_dev::components::button::*;
use corviraptor_dot_dev::components::*;

use crate::main_site::pages;

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <SectionTitle title={"// software"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>
                    <LinkButton name={ "codeberg" } url={ "https://codeberg.org/corviraptor" } icon={ IconType::NerdFont("nf-linux-codeberg".to_string()) } />

                    <LinkButton name={ "github" } url={ "https://github.com/corviraptor" } icon={ IconType::NerdFont("nf-fa-github".to_string()) } />
                </div>
            }}/>

            <SectionTitle title={"// stuff"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>
                    <LinkButton name={ "bandcamp" } url={ "https://corviraptor.bandcamp.com" } icon={ IconType::NerdFont("nf-fa-bandcamp".to_string()) } />

                    <LinkButton name={ "twitch" } url={ "https://www.twitch.tv/corviraptor" } icon={ IconType::NerdFont("nf-fa-twitch".to_string()) } />

                    <LinkButton name={ "youtube" } url={ "https://www.youtube.com/@corviraptor" } icon={ IconType::NerdFont("nf-fa-youtube_play".to_string()) } />

                    <LinkButton name={ "ko-fi" } url={ "https://ko-fi.com/corviraptor" } icon={ IconType::Inline(icons::KOFI.to_string()) } />
                </div>
            }}/>

            <SectionTitle title={"// socials"} subtitle={"i barely use social media these days, but here you go"} />
            <Section content={ html!{
                <div class={ "content-button-wrapper" }>
                    <LinkButton name={ "linkedin" } url={ "https://www.linkedin.com/in/katy-winter/" } icon={ IconType::NerdFont("nf-md-linkedin".to_string()) } />

                    <LinkButton name={ "mastodon" } url={ "https://mastodon.gamedev.place/@corviraptor" } icon={ IconType::NerdFont("nf-md-mastodon".to_string()) } />

                    <LinkButton name={ "bsky" } url={ "https://bsky.app/profile/corviraptor.bsky.social" } icon={ IconType::Inline(icons::BSKY.to_string()) } />

                    <LinkButton name={ "tumblr" } url={ "https://corviraptor.tumblr.com/" } icon={ IconType::NerdFont("nf-fa-tumblr".to_string()) } />

                    <LinkButton name={ "twitter" } url={ "https://twitter.com/corviraptor" } icon={ IconType::NerdFont("nf-fa-twitter".to_string()) } />
                </div>
            }}/>
        </div>
    };
    pages::build_page(Some("links".to_string()), None, content)
}
