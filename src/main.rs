#![allow(non_snake_case)]

mod words;
use dioxus::events::MouseEvent;
use dioxus::html::input_data::keyboard_types::Code;
use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon, IconButton};
use words::*;

#[derive(Clone, Copy)]
enum MainPanel {
    Typing,
    Info,
}

#[derive(Clone, Copy)]
enum TypingSide {
    Left,
    Right,
}

pub(crate) struct AppState {
    keyboard: KeyboardState,
    words: WordData,
    panel: MainPanel,
    side: TypingSide,
}
impl AppState {
    pub(crate) fn new(dict: &WordDictionary) -> Self {
        AppState {
            keyboard: KeyboardState::new(dict),
            words: WordData::new(10, dict),
            panel: MainPanel::Typing,
            side: TypingSide::Left,
        }
    }
}

fn App(cx: Scope) -> Element {
    use_context_provider(&cx, AppDictionaries::default);
    let dict = use_context::<AppDictionaries>(&cx)?;

    use_context_provider(&cx, || AppState::new(&dict.read().left));
    let app = use_context::<AppState>(&cx)?;

    let panel = match app.read().panel {
        MainPanel::Typing => rsx! { TypingWindow { } },
        MainPanel::Info   => rsx! { InfoWindow { } },
    };

    cx.render(rsx!{
        div {
            class: "h-screen flex bg-gradient-to-t from-stone-900 via-gray-700 to-gray-500 bg-gradient-to-u
            text-white",
            tabindex: "-1",
            onkeydown: move |evt| {
                let key_code = &evt.code();
                match key_code {
                    Code::Backspace => { app.write().words.pop(); },
                    Code::Space => { app.write().words.submit(); },
                    Code::Enter => { app.write().words.submit(); },
                    _ => ()
                }

                if app.read().words.buffer().is_empty() {
                    let dict = dict.read();
                    let dictionary = match app.read().side {
                        TypingSide::Left => &dict.left,
                        TypingSide::Right => &dict.right,
                    };

                    app.write().words = WordData::new(10, dictionary);
                }

            },
            onkeypress: move |evt| {
                let key = &evt.key();
                app.write().keyboard.update_for(&KeyState::new(key, true));

                if let Key::Character(key) = key {
                    app.write().words.push_str(&key.to_string());
                };
            },
            onkeyup: move |evt| {
                let key = &evt.key();
                app.write().keyboard.update_for(&KeyState::new(key, false));
            },
            div { class: "basis-1/4"}
            div { class: "basis-1/2",
                TopBar { }
                panel 
            }
            div { class: "basis-1/4"}
        }
    })
}

fn TopBar(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let dict = use_context::<AppDictionaries>(&cx)?;
    let flip_side = move |_| {
        let side = app.read().side;
        match side {
            TypingSide::Left => {
                app.write().side = TypingSide::Right;
                app.write().keyboard = KeyboardState::new(&dict.read().right);
            },
            TypingSide::Right => {
                app.write().side = TypingSide::Left;
                app.write().keyboard = KeyboardState::new(&dict.read().left);
            },
        };
        app.write().words.drain();
    };
    let toggle_info = move |_| {
        let panel = &mut app.write().panel;
        *panel = match panel {
            MainPanel::Typing => MainPanel::Info,
            MainPanel::Info => MainPanel::Typing,
        }
    };

    cx.render(rsx!(
        div {
            class: "flex justify-between items-center m-5 font-semibold",
            div {
                a {
                    href: "#",
                    h1 {
                        class: "text-xl font-bold tracking-tight leading-none
                        text-gray-900 md:text-4xl lg:text-4xl dark:text-white",
                        mark {
                            class: "px-2 text-white bg-gray-400 rounded dark:bg-gray-600",
                            "Hemi"
                        }
                        "Typer"
                    }
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
        title: "Toggle Info",
        fill: "white",
        disabled: false,
        size: 24,
        icon: Shape::InformationCircle,
    }))
}

fn TypingWindow(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let app = app.write();

    let next = app.words.next_word().unwrap_or(" ");
    let prev = app.words.last_word();
    let current = app.words.input();

    cx.render(rsx!(
        div {
            class: "flex justify-center items-center content-center gap-5 p-10 mt-40 h-32",
            h2 { class: "basis-1/4 text-right",                                              "{prev}" }
            h1 { class: "text-xl font-bold tracking-tight text-white basis-1/4 text-center", "{current}" }
            h2 { class: "basis-1/4 text-left",                                               "{next}" }
        }
        Keyboard { }
    ))
}

fn InfoWindow(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col justify-center items-center content-center gap-5 p-10 mt-40",
            div {
                class: "w-1/2 text-center",
                h1 { class: "text-xl tracking-tight text-white font-bold", "what" }
                p { class: "text-left",
                    "HemiTyper is an experimental typing tutor that allows you to train 
                    typing speed of your hands separately, providing you with only half 
                    the keyboard per training session."
                } 
                br { }
            }

            div {
                class: "w-1/2 text-center mt-5",
                h1 { class: "text-xl tracking-tight text-white font-bold", "why" }
                p { class: "text-left",
                    "I've found that training raw typing speed this way yields 
                    great results long-term, but there wasn't many typing tutors that 
                    offer this kind of training - so I made one." 
                } 
                br { }
                div { 
                    class: "mt-20",
                    "made with ❤ by ",
                    span { class: "underline decoration-blue-500", a { href: "https://lectro.moe/", "lectro.moe"} }
                }
            }
        }
    ))
}

fn Keyboard(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let keyboard_state = &app.write().keyboard;

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
