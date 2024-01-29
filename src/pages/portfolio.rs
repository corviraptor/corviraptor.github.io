use yew::prelude::*;
use crate::{pages, components::*};

#[function_component]
pub fn Content() -> Html {
    let content = html! {
        <div>
            <SectionTitle title={"// gamedev"} />
            <Section content={ html!{
                <div>
                    <h2> { "Vesper" } </h2>
                    <p> { "Vesper is the working title of a game project I'm working on. It's a Stylish Action FPS game that combines the accessible yet deep and expressive
                    gameplay mechanics of Stylish Action games like Metal Gear Rising: Revengeance and Devil May Cry with the intuitive controls and perspective of FPS games.
                    The game also takes a lot of inspiration from quake-like FPS games like Half Life and Team Fortress 2, bearing similar expressive movement mechanics and
                    integrating them into the combo system of the game." } </p> 
                    <iframe width="420" height="315"
                        src="https://www.youtube.com/embed/p5QWRLMQp5k">
                    </iframe> 
                </div>
            }}/>

            <SectionTitle title={"// music"} />
            <Section content={ html!{
                <div>
                    <h2> { "ANNIHILATRIX (Albums)" } </h2>
                    <p> { "ANNIHILATRIX is a pair of albums based on the story of a game pitch that's hanging around my head. They're both avaliable on Bandcamp!" } </p>
                    <div class={ "content-button-wrapper" }>
                    <ContentButton name={ "ANNIHILATRIX Part I" } url={ "https://corviraptor.bandcamp.com/album/annihilatrix-part-i" } />
                    <ContentButton name={ "ANNIHILATRIX Part II" } url={ "https://corviraptor.bandcamp.com/album/annihilatrix-part-ii" } />
                    </div>
                </div>
            }}/>
        </div>
    };
    pages::build_page(Some("portfolio"), Some("here's a collection of stuff I've worked on!".to_string()), content)
}