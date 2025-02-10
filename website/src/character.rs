use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

use crate::{props, Route};

#[component]
pub fn Character(id: String) -> Element {
    let character = use_persistent(id, || props::Character::new());

    let c = character.read();

    rsx! {
        div {
            h1 { "Character Sheet" }
            div { "name" } input { r#type: "text", value: "{c.name}" }
        }
    }
}

#[component]
pub fn CharactersList() -> Element {
    let create_character = || {
        let new_id = rand::random::<u128>();

        use_persistent(new_id, || props::Character::new());

        navigator().push(Route::Character {
            id: new_id.to_string(),
        });
    };

    let upload_character = move |e: Event<FormData>| {
        let file_engine = e.data.files().expect("file input missing files");
        let mut files = file_engine.files();
        if files.is_empty() {
            return;
        }
        assert_eq!(files.len(), 1, "Only one character file can be uploaded at a time");
        let file = files.remove(0);
        tracing::info!("Character file uploaded: {:?}", file);

        spawn(async move {
            let file_bytes = file_engine.read_file(&file).await.expect("one uploaded file content missing");
            match serde_json::from_slice::<props::Character>(&file_bytes) {
                Err(err) => {
                    tracing::error!("Failed to parse character file: {:?}", err);
                }
                Ok(character) => {
                    let new_id = rand::random::<u128>();
                    use_persistent(new_id, || character);
                    navigator().push(Route::Character {
                        id: new_id.to_string(),
                    });
                }
            }
        });
    };

    rsx! {
        div {
            h1 { "Characters List" }
            button { onclick: move |_| create_character(), title: "Create new character to edit", name: "new_character", "New Character" }
            input { r#type: "file", title: "Upload", name: "upload_character", accept: "application/json", oninput: upload_character, "Upload Character" }
        }
    }
}
