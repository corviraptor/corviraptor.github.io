use yew::prelude::*;

use corviraptor_dot_dev::components::button::*;
use corviraptor_dot_dev::components::*;

use crate::main_site::pages;

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>

            <SectionTitle title={"// buttons"} />
            <Section content={ html!{
                <>
                    <h2> { "screen buttons" } </h2>

                    <div class={ "content-button-wrapper" }>
                        <LinkButton name={ "button with custom svg icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::Inline(icons::KOFI.to_string()) } />

                        <LinkButton name={ "button with nerdfont icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } />
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                    </div>

                    <h2> { "physical buttons" } </h2>

                    <div class={ "content-button-wrapper" }>
                        <LinkButton name={ "button with custom svg icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::Inline(icons::KOFI.to_string()) } style={ ButtonStyle::Physical }/>

                        <LinkButton name={ "button with nerdfont icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                    </div>

                    <h2> { "icon buttons" } </h2>

                    <div class={ "content-button-wrapper" }>
                        <IconButton name={ "screen icon button" } action={ButtonAction::Url("https://corviraptor.github.io/".to_owned())} icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Screen } />
                        <IconButton name={ "disabled screen icon button" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Screen } />
                        <IconButton name={ "physical icon button"} action={ButtonAction::Url("https://corviraptor.github.io/".to_owned())} icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Physical } />
                        <IconButton name={ "disabled physical icon button"} icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Physical } />
                    </div>
                </>
            }}/>
        </div>
    };
    pages::build_page(Some("components test".to_string()), None, content)
}
