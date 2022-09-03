#![allow(non_snake_case)]

mod text_container;
use dioxus::prelude::*;
use std::{fs::File, io::Read};

struct WordDictionary {
    buffer: Vec<String>,
}
impl WordDictionary {
    pub(crate) fn new(path: &str) -> Self {
        let mut file_string = String::new();
        File::open(path)
            .expect(&format!("Couldn't open path {}", path))
            .read_to_string(&mut file_string)
            .expect("Couldn't read file contents");
        let buffer = serde_json::from_str(&file_string).expect("Couldn't parse the dictionary");
        WordDictionary { buffer }
    }
}

fn App(cx: Scope) -> Element {
    let dictionary = use_context_provider(&cx, || WordDictionary::new("/assets/dictionary"));
    let text = use_context_provider(&cx, || {});
    cx.render(rsx!(
        div {
            class: "h-screen flex bg-gradient-to-t from-stone-900 via-gray-700 to-gray-500 bg-gradient-to-u
            text-white",
            div { class: "basis-1/4"}
            div {
                class: "basis-1/2",
                TopBar { }
                TextWindow { }
                Keyboard { }
            }
            div { class: "basis-1/4"}
        }
    ))
}

fn TopBar(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex justify-between items-center m-5 font-semibold",
            h1 {
                class: "text-xl font-bold tracking-tight leading-none
                text-gray-900 md:text-4xl lg:text-4xl dark:text-white",
                mark { class: "px-2 text-white bg-gray-400 rounded dark:bg-gray-600", "Hemi"}
                "Typer"
            }
            div { class: "", " " }
            div {
                class: "",
                div { "About" }
                div { "Change side" }
            }
        }
    ))
}

fn TextWindow(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex justify-center items-center content-center gap-5 p-20 mt-40",
            p {
                class: "basis-1/4 text-right",
                "Previous"
            }
            h1 {
                class: "text-xl font-bold tracking-tight text-white basis-1/4 text-center",
                "Current" }
            p {
                class: "basis-1/4 text-left",
                "Next"
            }
        }
    ))
}

fn Keyboard(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col items-center content-center p-10 mt-40",
            div { "1 row" }
            div { "2 row" }
            div { "3 row" }
        }
    ))
}

fn main() {
    dioxus::web::launch(App);
}
