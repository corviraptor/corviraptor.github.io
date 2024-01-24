use yew::prelude::*;
use crate::components::*;

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
            <div class={ "section" }>
                <div class={ "content-button-wrapper" }>
                    
                    <a href="https://corviraptor.bandcamp.com" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "bandcamp" }</h3>
                    </a>

                    <a href="https://www.twitch.tv/corviraptor" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "twitch" }</h3>
                    </a>

                    <a href="https://github.com/corviraptor" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "github" }</h3>
                    </a>

                    <a href="https://www.linkedin.com/in/katy-winter/" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "linkedin" }</h3>
                    </a>

                    <a href="https://cohost.org/corviraptor" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "cohost" }</h3>
                    </a>

                    <a href="https://ko-fi.com/corviraptor" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "ko-fi" }</h3>
                    </a>

                    <a href="https://twitter.com/corviraptor" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "twitter" }</h3>
                    </a>
                </div>
            </div>
        </div>
    }
}