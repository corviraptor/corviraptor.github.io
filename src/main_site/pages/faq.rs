use crate::main_site::pages;
use corviraptor_dot_dev::markdown::*;
use yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {}
}

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <Markdown file={ MarkdownFile::Relative("faq".to_string())} style={ MarkdownStyle::Section } />
    };
    pages::build_page(Some("faq".to_string()), None, content)
}
