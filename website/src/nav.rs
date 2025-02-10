use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navigation() -> Element {
    rsx! {
        div {
            class: "sidenav",
            Link { to: Route::Home { }, "Home" }
            Link { to: Route::CharactersList, "Characters" }
        }
        Outlet::<Route> {}
    }
}
