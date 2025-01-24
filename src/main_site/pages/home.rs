use crate::main_site::pages;
use corviraptor_dot_dev::components::*;

use yew::prelude::*;

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <>
        <Section content={ html!{
            <div>
                <img src="images/katy-image.jpg" alt="A picture of Katy Winter's face." style="float:right;width:25%;margin:1em;" />

                <p> { "My name is Kathrynne Alcina Winter, also known by the handle " } <q> {"corviraptor"} </q> { " and whatever shortening of Kathrynne you prefer. I do gamedev, music production,
                programming, 3D animation, concept art, and a bunch more stuff. I also carry a broad range of other interests with me that 
                find their way into my work. In the end, I just want to make things people enjoy, but my work is often also a reflection of 
                myself and what I'm passionate about." } </p>

                <p> { "I'm particularly enamored with computer science and software. I spend a lot of time challenging myself with new programming projects and
                experimenting with my computers, and I'll always take whatever opportunity I can to learn more. I'm a very passionate programmer, and I deeply
                value working with and learning from others who've taken a different path than me." } </p>

                <p> { "Beyond software development and gamedev, I do music production and draw in my spare time, in addition to hanging out and playing video games
                with my friends. I also spend a lot of time learning about math, technology, and loads of other things." } </p>
            </div>
        }}/>
        <SectionTitle title={"// gamedev"} />
        <Section content={ html!{
            <div>
                <p> { "I grew up on Source Engine games, so I have a deep-seated passion for the FPS genre. More recently, I have
                fallen in love with Stylish Action games as well and find the way they balance challenging gameplay with
                accessibility fascinating. Beyond these, I have a soft spot for games that just make me feel a strong or unique 
                emotion in some way, regardless of how they accomplish it." } </p>

                <p> { "I tend to be pretty critical of the games I play, but I'm most critical of the
                games I love. I'm obsessed with picking apart the game design decisions that make me feel a certain way,
                and how they support or undermine the core experience the developers wanted to deliver." } </p>

                <p> {  "Eventually, that led me to game development as a career goal. In many cases when I feel something in a game,
                I'll say to myself something along the lines of " } <q> { "I want to make something that makes people feel like this." } </q>
                { " This underpins a lot of my creative endeavors; I'm not afraid to wear my inspirations on my sleeve and I deeply
                enjoy the experience of making something that makes people feel a certain way." } </p>
            </div>
        }}/>
        </>
    };
    pages::build_page(
        Some("hello".to_string()),
        Some("welcome to my website!".to_string()),
        content,
    )
}
