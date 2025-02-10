use dioxus::{logger::tracing::info, prelude::*, router::prelude::*};

mod markdown;
mod nav;
mod props;
mod character;
mod world;

use self::{
    character::{Character, CharactersList},
    markdown::Markdown,
    nav::Navigation,
    world::World,
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

        #[route("/world")]
        World { },

        #[route("/characters")]
        CharactersList,

        #[route("/character/:id")]
        Character {
            id: String,
        },
    #[end_layout]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn Home() -> Element {
    let home_md = use_resource(|| async move {
        reqwest::get("https://zakarumych.github.io/my-dnd-world/resources/articles/home.md")
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    });

    rsx! {
        Markdown {
            content: home_md.cloned().unwrap_or_default(),
        }
    }
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
