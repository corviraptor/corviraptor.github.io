use crate::{components::*, pages};
use yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {}
}

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <SectionTitle title={ "// pronouns?" } />
            <Section content={ html!{
                <p>
                    {
                        "She/They/It!"
                    }
                </p>
            }}/>


            <SectionTitle title={ "// what's that font you always use?" } />
            <Section content={ html!{
                <div>
                    <p>
                        <a href="https://github.com/be5invis/Iosevka" target="_blank" rel="noopener noreferrer">
                            { "Iosevka Slab" }
                        </a>

                        {
                            ", a free and open-source programming typeface. I originally found it while looking for nice-looking condensed
                            monospace fonts to use in my text editor, but I quickly fell in love with it and use it all over the place now,
                            even in places where proportional fonts are normally used. I originally used the standard version, but slab-serif 
                            has really grown on me." 
                        }
                    </p>
                    <p>
                        {
                            "I find it easy to read compared to many other fonts, although I don't have any empirical
                            evidence that it improves accessibility. Also, not having to worry about liscensing is nice. I actually compiled
                            this customized version of the font from the source code, mainly because I like how this 3 looks. " 
                        }
                    </p>
                </div>
            }}/>

            <SectionTitle title={ "// how'd you make this website?" } />
            <Section content={ html!{
                <div>
                    <p>
                        {
                            "I made this website with WebAssembly and "
                        }

                        <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer">
                            { "Yew" }
                        </a>

                        {
                            ", mostly just as an excuse to learn Rust, HTML, and CSS. It's hosted on Github Pages, which is why I'm using a hash
                            router instead of a normal browser router. If you're wondering why it looks so blocky and chunky, I just think it looks cute."
                        }
                    </p>
                </div>
            }}/>
        </div>
    };
    pages::build_page(Some("faq".to_string()), None, content)
}
