use dioxus::{logger::tracing::info, prelude::*, router::prelude::*};

mod character;
mod markdown;
mod nav;

use self::{
    character::{Character, CharactersList},
    markdown::Markdown,
    nav::Navigation,
};

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[layout(Navigation)]
        #[route("/")]
        Home { },

        #[nest("/character")]
            #[route("/")]
            CharactersList,

            #[route("/:id")]
            Character {
                id: String,
            },
        #[end_nest]
    #[end_layout]

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn Home() -> Element {
    rsx! {
        h1 { "Home Page" }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page Not Found" }

        Link { to: Route::Home { }, "Go Home" }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            class: "main",
            Router::<Route> { }
        }
    }
}
