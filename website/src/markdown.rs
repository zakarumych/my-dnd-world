use dioxus::prelude::*;
use pulldown_cmark::Parser;

/// Render some text as markdown.
#[component]
pub fn Markdown(url: String) -> Element {
    let content = use_resource(move || {
        let url = url.clone();
        async move { reqwest::get(url).await.unwrap().text().await.unwrap() }
    });

    let content = content.cloned().unwrap_or_default();

    let parser = Parser::new(&content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div { class: "md-content", dangerous_inner_html: "{html_buf}" }
    }
}
