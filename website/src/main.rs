use std::fmt::Display;

use dioxus::{logger::tracing::info, prelude::*, router::prelude::*};

mod character;
mod markdown;
mod nav;
mod props;

use self::{
    character::{Character, CharactersList},
    markdown::Markdown,
    nav::Navigation,
};

const MAIN_CSS: Asset = asset!("/assets/tailwind.css");
const MD_CSS: Asset = asset!("/assets/md.css");

fn main() {
    dioxus::launch(App);
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(Navigation)]
        #[route("/")]
        Home { },

        #[route("/characters")]
        CharactersList,

        #[route("/character/:id")]
        Character {
            id: String,
        },

        #[route("/article/:..segments")]
        Article {
            segments: Vec<String>,
        },
    #[end_layout]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
pub fn Article(segments: Vec<String>) -> Element {
    let path = join_strings(segments.iter(), '/').unwrap_or_else(|| "index".to_string());
    let url = if path.ends_with(".md") {
        format!("https://raw.githubusercontent.com/zakarumych/my-dnd-world/refs/heads/main/resources/articles/{path}")
    } else {
        format!("https://raw.githubusercontent.com/zakarumych/my-dnd-world/refs/heads/main/resources/articles/{path}.md")
    };

    rsx! {
        Markdown { url }
    }
}

#[component]
fn Home() -> Element {
    rsx! {}
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "font-sans flex flex-col justify-center items-stretch gap-10 h-screen w-screen text-3xl bg-gray-300",
            span {
                class: "flex justify-center px-8 py-4 text-gray-100 bg-black",
                "Page Not Found"
            }
            Link {
                class: "flex justify-center px-8 py-4 bg-gray-100 hover:bg-amber-200",
                to: Route::Home { },
                "Go Home",
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: MD_CSS }
        meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
        Router::<Route> { }

    }
}

fn join_strings<T>(mut strings: impl Iterator<Item = T>, separator: impl Display) -> Option<String>
where
    T: Display,
{
    use std::fmt::Write;

    let first = strings.next()?;
    let lower = strings.size_hint().0;

    let mut result = String::with_capacity(lower);
    write!(result, "{}", first).unwrap();

    for item in strings {
        write!(result, "{separator}{item}").unwrap();
    }

    Some(result)
}
