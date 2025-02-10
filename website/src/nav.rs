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
                Link {
                    class: "flex justify-center px-8 py-4 bg-gray-100 hover:bg-amber-200",
                    to: Route::World { },
                    "World"
                }
            }

            div {
                class: "flex flex-1 overflow-auto",
                // Main content
                Outlet::<Route> {}
            }
        }
    }
}
