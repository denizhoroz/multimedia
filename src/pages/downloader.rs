use dioxus::prelude::*;

#[component]
pub fn Downloader() -> Element {
    rsx! {
        div { class: "content-area",
            h1 { "Downloader" }
            div { class: "content-box" }
            // Downloader-specific UI goes here
        }
    }
}