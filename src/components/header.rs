use yew::prelude::*;

use crate::HOME_URL;

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderProps {
    pub name: String,
}

#[function_component]
pub fn Header() -> Html {
    let home = AttrValue::from(HOME_URL);
    html! {
        <header class={ classes!("header-container") }>
            <div class={ "left-header-container" }>
                <a href={home.clone()}>
                    <div class={ "logo-container" } >
                        <div class={ "header-title" }>
                            <h2 style="display: table-caption;">{ "katy winter" }</h2>
                        </div>
                    </div>
                </a>
                <a href={home}>
                    { return_home_button() }
                </a>
            </div>
            <div class={ "contact-container" }>
                // <h2>{ "kathrynne@corviraptor.net" }</h2> TODO: uncomment this when i actually own the domain and have the email set up
            </div>
        </header>
    }
}

pub fn return_home_button() -> Html {
    html! {
        <div class={ "return-home" }>
            <h4>{ "return home" }</h4>
        </div>
    }
}
