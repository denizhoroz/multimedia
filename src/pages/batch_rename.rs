use dioxus::prelude::*;

#[component]
pub fn BatchRename() -> Element {
    rsx! {
        div { class: "content-area",
            h1 { "Batch Rename" }
            div { class: "content-box" }
            // Batch rename UI goes here
        }
    }
}