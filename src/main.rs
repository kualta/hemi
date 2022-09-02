#![allow(non_snake_case)]
mod components;
use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
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
            class: "flex justify-between items-center m-5",
            h1 {
                class: "text-xl font-bold tracking-tight leading-none text-gray-900 md:text-4xl lg:text-4xl dark:text-white",
                mark { class: "px-2 text-white bg-pink-400 rounded dark:bg-pink-500", "Hemi"} 
                "Typer"
            }
            div { class: "", "Change Side" }
            div { class: "", "About" }
        }
    ))
}

fn TextWindow(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex justify-center items-center content-center gap-5 p-20",
            p { "Previous" }
            h1 {
                class: "text-xl font-bold tracking-tight text-white",
                "Current" }
            p { "Next" }
        }
    ))
}

fn Keyboard(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col items-center content-center p-10",
            div { "1 row" }
            div { "2 row" }
            div { "3 row" }
        }
    ))
}

fn main() {
    dioxus::web::launch(app);
}
