use dioxus::{logger::tracing::info, prelude::*};

mod markdown;

use self::markdown::Markdown;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let article = use_resource(|| async move {
        reqwest::get("https://raw.githubusercontent.com/zakarumych/my-dnd-world/refs/heads/main/articles/hello.md")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Markdown { content: article.unwrap() }
    }
}
