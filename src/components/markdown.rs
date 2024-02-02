use std::path::PathBuf;

use yew::{platform::spawn_local, prelude::*};

use gloo::net::http::*;

const MD_DIR: &str = "markdown";

#[derive(Clone, PartialEq, Properties)]
pub struct MarkdownProps {
    pub file: MarkdownFile,

    #[prop_or(None)]
    pub header: Option<String>,
}

#[derive(Clone, PartialEq)]
pub enum MarkdownFile {
    Relative(String),
    Readme,
}

#[function_component]
pub fn Markdown(props: &MarkdownProps) -> Html {
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

    let mut markdown: String = state.to_string();

    markdown = match &props.header {
        None => markdown.to_string(),
        Some(x) => trim_to_header(markdown, x),
    };

    let parser = pulldown_cmark::Parser::new(&markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    Html::from_html_unchecked(html_output.into())
}

fn trim_to_header(markdown: String, header: &str) -> String {
    markdown
        .split("# ")
        .find(|x| x.contains(header))
        .unwrap_or("Loading...")
        .replace(header, "")
        .trim()
        .to_string()
}
