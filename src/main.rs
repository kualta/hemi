#![allow(non_snake_case)]

mod words;
use dioxus::events::MouseEvent;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon, IconButton};
use words::*;

enum MainPanel {
    Typing,
    Info,
}

enum TypingSide {
    Left,
    Right,
}

fn App(cx: Scope) -> Element {
    let left_dictionary = use_state(&cx, init_left_dictionary);
    let right_dictionary = use_state(&cx, init_right_dictionary);
    use_context_provider(&cx, || WordData::new(10, left_dictionary));
    use_context_provider(&cx, || KeyboardState::new(left_dictionary));
    use_context_provider(&cx, || MainPanel::Typing);
    use_context_provider(&cx, || TypingSide::Left);

    let word_buffer = use_context::<WordData>(&cx)?;
    let keyboard_state = use_context::<KeyboardState>(&cx)?;
    let side = use_context::<TypingSide>(&cx)?;

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

                if word_buffer.buffer().is_empty() {
                    let dictionary = match *side.read() {
                        TypingSide::Left => left_dictionary,
                        TypingSide::Right => right_dictionary,
                    };

                    *word_buffer = WordData::new(10, dictionary);
                }

            },
            onkeypress: move |evt| {
                let key = &evt.key();
                keyboard_state.write().update_for(&KeyState::new(key, true));

                if let Key::Character(key) = key {
                    word_buffer.write().push_str(&key.to_string());
                };
            },
            onkeyup: move |evt| {
                let key = &evt.key();
                keyboard_state.write().update_for(&KeyState::new(key, false));
            },
            div { class: "basis-1/4"}
            div { class: "basis-1/2",
                TopBar { }
                TextWindow { word_data: word_buffer }
                Keyboard { }
            }
            div { class: "basis-1/4"}
        }
    ))
}

fn TopBar(cx: Scope) -> Element {
    let word_buffer = use_context::<WordData>(&cx)?;
    let side = use_context::<TypingSide>(&cx)?;
    let flip_side = move |_| {
        let mut side = side.write();
        *side = match *side {
            TypingSide::Left => TypingSide::Right,
            TypingSide::Right => TypingSide::Left,
        };
        word_buffer.write().drain();
    };
    let panel = use_context::<MainPanel>(&cx)?;
    let toggle_info = move |_| {
        let mut panel = panel.write();
        *panel = match *panel {
            MainPanel::Typing => MainPanel::Info,
            MainPanel::Info => MainPanel::Typing,
        }
    };

    cx.render(rsx!(
        div {
            class: "flex justify-between items-center m-5 font-semibold",
            div {
                h1 {
                    class: "text-xl font-bold tracking-tight leading-none
                    text-gray-900 md:text-4xl lg:text-4xl dark:text-white",
                    mark { class: "px-2 text-white bg-gray-400 rounded dark:bg-gray-600", "Hemi"}
                    "Typer"
                }
            }
            div { " " }
            div {
                ChangeSideButton { onclick: flip_side }
                InfoButton { onclick: toggle_info }
            }
        }
    ))
}

#[inline_props]
fn ChangeSideButton<'a>(cx: Scope, onclick: EventHandler<'a, MouseEvent>) -> Element {
    cx.render(rsx!(IconButton {
        onclick: move |evt| {
            onclick.call(evt);
        },
        class: "pt-3 ml-5",
        title: "Flip Typing Side",
        fill: "white",
        disabled: false,
        size: 24,
        icon: Shape::Refresh,
    }))
}

#[inline_props]
fn InfoButton<'a>(cx: Scope, onclick: EventHandler<'a, MouseEvent>) -> Element {
    cx.render(rsx!(IconButton {
        onclick: move |evt| {
            onclick.call(evt);
        },
        class: "pt-3 ml-5",
        title: "Flip Typing Side",
        fill: "white",
        disabled: false,
        size: 24,
        icon: Shape::InformationCircle,
    }))
}

#[inline_props]
fn TextWindow<'a>(cx: Scope<'a>, word_data: UseSharedState<'a, WordData>) -> Element {
    let word_data = word_data.read();
    let main_panel = use_context::<MainPanel>(&cx)?;

    let panel = match *main_panel.read() {
        MainPanel::Typing => {
            let next = word_data.next_word().unwrap_or(" ");
            let prev = word_data.last_word();
            let current = word_data.input();

            rsx!(
                div {
                    class: "flex justify-center items-center content-center gap-5 p-10 mt-40 h-32",
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
    let keyboard_state = use_context::<KeyboardState>(&cx)?;
    let keyboard_state = keyboard_state.write();

    let button_active = "w-16 h-14 text-gray-300 bg-white border-2 border-gray-300 
    focus:outline-none focus:ring-4 focus:ring-gray-200 
    font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 
    dark:text-white dark:border-gray-600 dark:focus:ring-gray-700";

    let button_inactive = "w-16 h-14 text-gray-900 bg-white focus:outline-none focus:ring-4 
    focus:ring-gray-200 font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 
    dark:text-white dark:border-gray-600 dark:focus:ring-gray-700";

    let keyboard = rsx! { keyboard_state.keys().iter().enumerate().map(|(i, row)| {
            let row_indent = (i * 10).to_string();
            rsx! {
                span { class: "ml-{row_indent}" }
                row.iter().map(|key| {
                    let button_style = if key.enabled() { button_active } else { button_inactive };
                    rsx! {
                        button {
                            class: "{button_style}",
                            "type": "button",
                            "{key.key()}"
                        }
                    }
                })
                br { }
            }
        }
    )};

    cx.render(rsx!(div {
        class: "content-center text-center gap-5 p-10 mt-40",
        keyboard
    }))
}

fn main() {
    dioxus_web::launch(App);
}
