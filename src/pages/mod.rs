mod portfolio;
mod faq;
mod home;
mod links;
mod not_found;

use yew::prelude::*;

use crate::{ route::Route, components::* };

pub fn build_page(title: Option<String>, subtitle: Option<String>, content: Html) -> Html {
    html! {
        <>  
            if title.is_some() {
                <TitleBox title={"// ".to_string() + &title.unwrap().to_lowercase()} subtitle={ subtitle }/>
            }
            <div class={ "content" }>
                { content }
            </div>
        </>
    }
}

pub fn page_content_switch(route: Route) -> Html {
    match route {
        Route::Home => html!{ <home::Page/> },
        Route::Faq => html!{ <faq::Page/> },
        Route::Portfolio => html!{ <portfolio::Page/> },
        Route::Links => html!{ <links::Page/> },
        Route::NotFound => html!{ <not_found::Page/> },
    }
}