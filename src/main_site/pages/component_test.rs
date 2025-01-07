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
                        <LinkButton name={ "button with icon disabled" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } />
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } />
                        <LinkButton name={ "button disabled" } />
                    </div>

                    <h2> { "physical buttons" } </h2>

                    <div class={ "content-button-wrapper" }>
                        <LinkButton name={ "button with custom svg icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::Inline(icons::KOFI.to_string()) } style={ ButtonStyle::Physical }/>

                        <LinkButton name={ "button with nerdfont icon" } url={ "https://corviraptor.github.io/" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with no icon" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button!!" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button with icon disabled" } icon={ IconType::NerdFont("nf-fa-face_smile".to_string()) } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button2" } url={ "https://corviraptor.github.io/" } style={ ButtonStyle::Physical }/>
                        <LinkButton name={ "button disabled" } style={ ButtonStyle::Physical }/>
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

            <SectionTitle title={"// HTML Form"} />
            <Section content={ html!{
                <form>
                <div id="from">
                    <label for="name">{"from:"}</label>
                    <input type="text" id="name" name="user_name" />
                </div>

                <div id="reply">
                    <label for="mail">{"reply:"}</label>
                    <input type="email" id="mail" name="user_email" />
                </div>

                <div id="message">
                    <label for="msg">{"Your message:"}</label>
                    <textarea id="msg" name="user_message"></textarea>
                </div>

                <div>

                    <select name={ "font picker" } id={ "font picker" }>
                    <option value={ "System Default" }>{ "System Default" }</option>
                        <option value={ "Iosevka Slab Web" } selected={ true }>{ "Iosevka Slab Web (Monospace)" }</option>
                        <option value={ "Iosevka" }>{ "Iosevka (Monospace)" }</option>
                        <option value={ "Source Serif 4" }>{ "Source Serif 4 (Serif)" }</option>
                        <option value={ "Atkinson Hyperlegible" }>{ "Atkinson Hyperlegible (Sans-Serif)" }</option>
                    </select>
                </div>

                <div class="button">
                    <button type="submit">{"Send your message"}</button>
                </div>

                </form>
            }}/>

        </div>
    };
    pages::build_page(Some("components test".to_string()), None, content)
}
