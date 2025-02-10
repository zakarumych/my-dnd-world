use dioxus::prelude::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CharacterDesc {
    name: String,
    level: u32,
}

impl Default for CharacterDesc {
    fn default() -> Self {
        Self {
            name: "New Character".to_string(),
            level: 0,
        }
    }
}

#[component]
pub fn Character(id: String) -> Element {
    let desc = use_signal(CharacterDesc::default);

    rsx! {
        div {
            h1 { "Character Sheet" }
            div { "name" } input { r#type: "text", value: "{desc.read().name}" }
            div { "level" } input { r#type: "number" }
        }
    }
}

#[component]
pub fn CharactersList() -> Element {
    let new_character = |_| {
        let new_id = rand::random::<u128>();

        navigator().push(Route::Character {
            id: new_id.to_string(),
        });
    };

    rsx! {
        div {
            h1 { "Characters List" }
            button { onclick: new_character, "New Character" }
        }
    }
}
