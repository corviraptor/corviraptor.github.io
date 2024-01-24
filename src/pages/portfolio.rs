use yew::prelude::*;
use crate::components::*;

#[function_component]
pub fn Title () -> Html {
    html! {
        <TitleBox title={"// portfolio"} subtitle={"here's a collection of stuff I've worked on!"} />
    }
}

#[function_component]
pub fn Content () -> Html {
    html! {
        <div>
            <SectionTitle title={"// gamedev"} />
            <div class={ "section" }>
                <h2> { "Vesper" } </h2>
                <p> { "Vesper is the working title of a game project I'm working on. It's a Stylish Action FPS game that combines the accessible yet deep and expressive
                gameplay mechanics of Stylish Action games like Metal Gear Rising: Revengeance and Devil May Cry with the intuitive controls and perspective of FPS games.
                The game also takes a lot of inspiration from quake-like FPS games like Half Life and Team Fortress 2, bearing similar expressive movement mechanics and
                integrating them into the combo system of the game." } </p> 
                <iframe width="420" height="315"
                src="https://www.youtube.com/embed/p5QWRLMQp5k">
                </iframe> 
            </div>

            <SectionTitle title={"// music"} />
            <div class={ "section" }>
                <h2> { "ANNIHILATRIX (Albums)" } </h2>
                <p> { "ANNIHILATRIX is a pair of albums based on the story of a game pitch that's hanging around my head. They're both avaliable on Bandcamp!" } </p>
                <div class={ "content-button-wrapper" }>
                    <a href="https://corviraptor.bandcamp.com/album/annihilatrix-part-i" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "ANNIHILATRIX Part I" }</h3>
                    </a>
                </div>
                <div class={ "content-button-wrapper" }>
                    <a href="https://corviraptor.bandcamp.com/album/annihilatrix-part-ii" target="_blank" rel="noopener noreferrer" class={ "content-button" }>
                        <h3>{ "ANNIHILATRIX Part II" }</h3>
                    </a>
                </div>
            </div>
        </div>
    }
}