use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navigation() -> Element {
    rsx! {
        div {
            class: "flex h-screen w-screen",

            // Navigation sidebar
            nav {
                class: "flex flex-col px-8 py-4 gap-2 bg-gray-300 text-xl",
                Link {
                    class: "flex justify-center px-8 py-4 text-gray-100 bg-black",
                    to: Route::Home { },
                    "My DnD World"
                }
                Link {
                    class: "flex justify-center px-8 py-4 bg-gray-100 hover:bg-amber-200",
                    to: Route::CharactersList { },
                    "Characters"
                }
            }

            div {
                class: "flex flex-1 px-8 py-4 overflow-auto bg-gray-100 text-l",
                // Main content
                Outlet::<Route> {}
            }
        }
    }
}
