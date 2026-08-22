use dioxus::prelude::*;

#[component]
pub fn FileConverter() -> Element {
    rsx! {
        div { class: "content-area",
            h1 { "File Converter" }
            div { class: "content-box", }
        }
    }
}
