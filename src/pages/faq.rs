use yew::prelude::*;
use crate::components::*;

#[function_component]
pub fn Title () -> Html {
    html! {
        <TitleBox title={"// faq"} />
    }
}

#[function_component]
pub fn Content () -> Html {
    html! {
        <div>
            <SectionTitle title={ "// what's that font you always seem to use?" } />
            <div class={ "section" }>
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



            <SectionTitle title={ "// how'd you make this website?" } />
            <div class={ "section" }>
                <p>
                    { 
                        "I made this website with WebAssembly and " 
                    }

                    <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer">
                        { "Yew" }
                    </a>
                
                    { 
                        ", mostly just as an excuse to learn Rust, HTML, and CSS." 
                    }
                </p>
            </div>
        </div>
    }
}