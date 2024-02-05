use crate::{markdown::*, pages};
use yew::prelude::*;

#[function_component]
pub fn Title() -> Html {
    html! {}
}

#[function_component]
pub fn Page() -> Html {
    let content = html! {
        <div>
            <Markdown file={ MarkdownFile::Relative("faq".to_string())} style={ MarkdownStyle::Section } />
        </div>
    };
    pages::build_page(Some("faq".to_string()), None, content)
}
