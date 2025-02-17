use dioxus::prelude::*;
use dioxus_sdk::storage::use_persistent;

use crate::{props, Route};

#[component]
pub fn Character(id: String) -> Element {
    let character = use_persistent(id, || props::Character::new());

    let c = character.read();

    rsx! {
        div {
            class: "flex flex-col",
            div {
                class: "flex flex-1",

                div {
                    class: "flex flex-col",
                    span {
                        class: "text-3xl",
                        "Character"
                    }
                    div {
                        class: "flex gap-4",
                        div {
                            class: "text-5xl",
                            "{c.name}"
                        }
                        div {
                            class: "text-5xl",
                            "{c.total_level()}"
                        }
                    }
                }

                div {
                    class: "grid grid-cols-3 content-start gap-4",
                    input { r#type: "text", placeholder: "race & class" }
                    div { "2" }
                    div { "3" }
                    div { "4" }
                    div { "5" }
                    div { "6" }
                }
            }
        }
    }
}

#[component]
pub fn CharactersList() -> Element {
    let create_character = || {
        let new_id = rand::random::<u128>();

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
        assert_eq!(
            files.len(),
            1,
            "Only one character file can be uploaded at a time"
        );
        let file = files.remove(0);
        tracing::info!("Character file uploaded: {:?}", file);

        spawn(async move {
            let file_bytes = file_engine
                .read_file(&file)
                .await
                .expect("one uploaded file content missing");
            match serde_json::from_slice::<props::Character>(&file_bytes) {
                Err(err) => {
                    tracing::error!("Failed to parse character file: {:?}", err);
                }
                Ok(character) => {
                    let new_id = rand::random::<u128>();
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
