mod portfolio;
mod faq;
mod home;
mod links;
mod not_found;

use yew::prelude::*;

use crate::route::Route;

pub fn page_content_switch(route: Route) -> Html {
    // TODO: use traits to reduce the code duplication here
    match route {
        Route::Home => html!{ <home::Content/> },
        Route::Faq => html!{ <faq::Content/> },
        Route::Portfolio => html!{ <portfolio::Content/> },
        Route::Links => html!{ <links::Content/> },
        Route::NotFound => html!{ <not_found::Content/> },
    }
}

pub fn page_title_switch(route: Route) -> Html {
    match route {
        Route::Home => html!{ <home::Title/> },
        Route::Faq => html!{ <faq::Title/> },
        Route::Portfolio => html!{ <portfolio::Title/> },
        Route::Links => html!{ <links::Title/> },
        Route::NotFound => html!{ <not_found::Title/> },
    }
}