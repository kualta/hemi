#![allow(non_snake_case)]

mod words;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use words::*;

enum MainPanel {
    Typing,
    Info,
}

enum TypingSide {
    Left,
    Right,
}

// struct TextWindow {
//     previous: String,
//     words: Iter<&String>,
// }

fn App(cx: Scope) -> Element {
    use_context_provider(&cx, || MainPanel::Typing);
    use_context_provider(&cx, || TypingSide::Left);
    use_context_provider(&cx, WordBuffer::default);

    let word_buffer = use_context::<WordBuffer>(&cx)?;

    cx.render(rsx!(
        div {
            class: "h-screen flex bg-gradient-to-t from-stone-900 via-gray-700 to-gray-500 bg-gradient-to-u
            text-white",
            tabindex: "-1",
            onkeydown: move |evt| {
                let key_code = &evt.code();
                let mut word_buffer = word_buffer.write();
                match key_code {
                    Code::Backspace => { word_buffer.pop(); },
                    Code::Space => { word_buffer.submit(); },
                    Code::Enter => { word_buffer.submit(); },
                    _ => ()
                }
            },
            onkeypress: move |evt| {
                if let Key::Character(key) = &evt.key() {
                    word_buffer.write().push_str(&key.to_string());
                };
            },
            div { class: "basis-1/4"}
            div { class: "basis-1/2",
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
    let left_dictionary = use_state(&cx, init_left_dictionary);
    let right_dictionary = use_state(&cx, init_right_dictionary);
    // let left_buffer = use_state(&cx, || WordBuffer::from_dictionary(30, left_dictionary))
    //     .words
    //     .iter()
    //     .peekable();
    // let right_buffer = use_state(&cx, || WordBuffer::from_dictionary(30, right_dictionary))
    //     .words
    //     .iter()
    //     .peekable();
    let word_buffer = use_context::<WordBuffer>(&cx)?;
    let word_buffer = word_buffer.read();

    let main_panel = use_context::<MainPanel>(&cx)?;
    let panel = match *main_panel.read() {
        MainPanel::Typing => {
            let side = use_context::<TypingSide>(&cx)?;
            // let mut words = match *side.read() {
            //     TypingSide::Left => left_buffer,
            //     TypingSide::Right => right_buffer,
            // };

            let prev = word_buffer.last_word();
            let current = word_buffer.input();
            let next = "";

            rsx!(
                div {
                    class: "flex justify-center items-center content-center gap-5 p-20 mt-40",
                    h2 { class: "basis-1/4 text-right", "{prev}" }
                    h1 { class: "text-xl font-bold tracking-tight text-white basis-1/4 text-center", "{current}" }
                    h2 { class: "basis-1/4 text-left", "{next}" }
                }
            )
        }
        MainPanel::Info => {
            rsx!(
                div { "HemiTyper by lectromoe! "}
            )
        }
    };

    cx.render(panel)
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
    dioxus_web::launch(App);
}
