mod portfolio;

use yew::prelude::*;

use portfolio::Portfolio;


use crate::route::Route;


pub fn page_switch(route: Route) -> Html {
    match route {
        Route::Home => html!{},
        Route::Faq => html!{},
        Route::Portfolio => html!{ <Portfolio/> },
        Route::Links => html!{},
        Route::NotFound => html!{},
    }
}