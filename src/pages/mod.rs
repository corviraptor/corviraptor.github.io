mod portfolio;
mod faq;
mod home;
mod links;
mod not_found;

use yew::prelude::*;

use crate::route::Route;

pub fn page_switch(route: Route) -> Html {
    match route {
        Route::Home => html!{ <home::Content/> },
        Route::Faq => html!{ <faq::Content/> },
        Route::Portfolio => html!{ <portfolio::Content/> },
        Route::Links => html!{ <links::Content/> },
        Route::NotFound => html!{ <not_found::Content/> },
    }
}