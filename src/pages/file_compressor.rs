use dioxus::prelude::*;

#[component]
pub fn FileCompressor() -> Element {
    rsx! {
        div { class: "content-area",
            h1 { "File Compressor" }
            div { class: "content-box" }
            // File compressor UI goes here
        }
    }
}