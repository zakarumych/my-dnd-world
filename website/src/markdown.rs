
use dioxus::prelude::*;
use pulldown_cmark::Parser;

/// Render some text as markdown.
#[component]
pub fn Markdown(content: String) -> Element {
    let parser = Parser::new(&content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div { class: "md-content", dangerous_inner_html: "{html_buf}" }
    }
}
