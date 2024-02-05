use std::path::PathBuf;

use yew::{platform::spawn_local, prelude::*};

use gloo::net::http::*;

use crate::components::*;

const MD_DIR: &str = "markdown";

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownProps {
    pub file: MarkdownFile,

    #[prop_or(MarkdownStyle::Default)]
    pub style: MarkdownStyle,
}

#[derive(Clone, PartialEq)]
pub enum MarkdownFile {
    Relative(String),
    Readme,
}

#[derive(Clone, PartialEq)]
pub enum MarkdownStyle {
    Default,
    Section,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let state = use_state(|| "Failed to load markdown file!".to_string());
    let path_buf = match &props.file {
        MarkdownFile::Relative(x) => {
            let mut path = PathBuf::new();
            path.push(MD_DIR);
            path.push(x);
            path.set_extension("md");
            path
        }
        MarkdownFile::Readme => {
            let mut path = PathBuf::new();
            path.push("README");
            path.set_extension("md");
            path
        }
    };

    use_effect_with(
        (), // () means run once on component's mount (load, first render, ...)
        {
            let state = state.clone();
            move |()| {
                let state = state.clone();
                spawn_local(async move {
                    let resp = Request::get(path_buf.to_str().unwrap()).send().await;
                    let fetch_result = resp.unwrap().text().await.unwrap();
                    state.set(fetch_result);
                });
            }
        },
    );

    let markdown: String = state.to_string();

    match &props.style {
        MarkdownStyle::Default => style_as_default(markdown),
        MarkdownStyle::Section => style_as_sections(markdown),
    }
}

fn markdown_to_html_string(markdown: String) -> String {
    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output.trim().to_string()
}

fn style_as_default(markdown: String) -> Html {
    Html::from_html_unchecked(markdown_to_html_string(markdown.clone()).into())
}

fn style_as_sections(markdown: String) -> Html {
    let sections = into_sections(markdown.clone());

    let mut output = Vec::<Html>::new();
    for section in sections {
        let html_output = markdown_to_html_string(section.content.clone());
        output.push(html! {
                <div>
                    if let Some(x) = section.header.clone() {
                        <SectionTitle title={ x } />
                    }
                    <Section content={ Html::from_html_unchecked(html_output.into())} />
                </div>
        });
    }

    Html::from_iter(output)
}

struct MarkdownSection {
    header: Option<String>,
    content: String,
}

fn into_sections(markdown: String) -> Vec<MarkdownSection> {
    let sections = markdown.split("// ").skip(1);

    let mut sections_buf = Vec::<MarkdownSection>::new();
    for section in sections {
        let header = section
            .split('\n')
            .collect::<Vec<&str>>()
            .first()
            .map(|x| x.to_string());

        let content = {
            if let Some(x) = header.clone() {
                section.replace(&x, "")
            } else {
                section.to_string()
            }
        };
        sections_buf.push(MarkdownSection { header, content });
    }

    sections_buf
}

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownSectionProps {
    pub file: MarkdownFile,
}
