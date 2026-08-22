use dioxus::prelude::*;

mod pages;
use pages::{downloader, batch_rename, file_converter, file_compressor};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy, PartialEq)]
pub enum Category {
    Downloader,
    BatchRename,
    FileConverter,
    FileCompressor,
}


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let selected = use_signal(|| Category::Downloader);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div {
            class: "app-section",
            Navbar { selected }
            ContentArea { selected }
        }
    }
}


#[component]
pub fn Navbar(mut selected: Signal<Category>) -> Element {
    rsx! {
        div {
            class: "navbar",
            div {
                h1 { "multimedia" }
            }

            nav {
                id: "navbar-category-list",
                a { href: "#", onclick: move |e| {e.prevent_default(); selected.set(Category::Downloader)}, "Downloader" }
                a { href: "#", onclick: move |e| {e.prevent_default(); selected.set(Category::BatchRename)}, "Batch Rename" }
                a { href: "#", onclick: move |e| {e.prevent_default(); selected.set(Category::FileConverter)}, "File Converter" }
                a { href: "#", onclick: move |e| {e.prevent_default(); selected.set(Category::FileCompressor)}, "File Compressor" }
            }
        }
    }
}

#[component]
pub fn ContentArea(selected: ReadSignal<Category>) -> Element {
    rsx! {
        div {
            style: "flex: 1;",
            match selected() {
                Category::Downloader => rsx! { downloader::Downloader {  } },
                Category::BatchRename => rsx! { batch_rename::BatchRename {  } },
                Category::FileConverter => rsx! { file_converter::FileConverter {  } },
                Category::FileCompressor => rsx! { file_compressor::FileCompressor {  } },
            }
        }
    }
}