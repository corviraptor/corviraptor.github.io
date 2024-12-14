use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/faq")]
    Faq,
    #[at("/portfolio")]
    Portfolio,
    #[at("/links")]
    Links,
    #[not_found]
    #[at("/404")]
    NotFound,
}