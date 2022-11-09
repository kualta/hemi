#![allow(non_snake_case)]

mod words;

use dioxus::prelude::*;
use dioxus::{
    core::UiEvent,
    events::{KeyboardData, MouseEvent},
    html::input_data::keyboard_types::{Code, Key},
};
use dioxus_heroicons::{solid::Shape, Icon, IconShape};
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

pub(crate) struct AppSettings {
    sound_enabled: bool,
    status_enabled: bool,
    keyboard_enabled: bool,
}

pub(crate) struct AppState {
    keyboard: KeyboardState,
    settings: AppSettings,
    typer: TypingData,
    panel: MainPanel,
    side: TypingSide,
}

impl AppState {
    pub(crate) fn new(dict: &WordDictionary) -> Self {
        AppState {
            keyboard: KeyboardState::new(dict),
            typer: TypingData::new(10, dict),
            panel: MainPanel::Typing,
            side: TypingSide::Left,
            settings: AppSettings {
                sound_enabled: true,
                status_enabled: true,
                keyboard_enabled: true,
            },
        }
    }
}

fn App(cx: Scope) -> Element {
    use_context_provider(&cx, AudioLibrary::default);
    let audio = use_context::<AudioLibrary>(&cx)?;

    use_context_provider(&cx, AppDictionaries::default);
    let dict = use_context::<AppDictionaries>(&cx)?;

    use_context_provider(&cx, || AppState::new(&dict.read().left));
    let app = use_context::<AppState>(&cx)?;

    let on_key_down = move |event: UiEvent<KeyboardData>| {
        let key_code = event.code();
        let mut app = app.write();

        match key_code {
            Code::Backspace => {
                app.typer.pop();
            }
            Code::Space => {
                app.typer.submit();
            }
            Code::Enter => {
                app.typer.submit();
            }
            _ => (),
        }

        if app.typer.buffer().is_empty() {
            let app_dict = dict.read();
            let dictionary = match app.side {
                TypingSide::Left => &app_dict.left,
                TypingSide::Right => &app_dict.right,
            };
            app.typer = TypingData::new(10, dictionary);
        }

        if app.settings.sound_enabled {
            audio.read().play(key_code);
        }
    };

    let on_key_press = move |event: UiEvent<KeyboardData>| {
        let key = &event.key();
        let mut app = app.write();
        app.keyboard.update_for(&KeyState::new(key, true));

        if let Key::Character(key) = key {
            app.typer.push_str(&key.to_string());
        };
    };

    let on_key_up = move |event: UiEvent<KeyboardData>| {
        let key = &event.key();
        app.write().keyboard.update_for(&KeyState::new(key, false));
    };

    let panel = match app.read().panel {
        MainPanel::Typing => rsx! { TypingWindow { } },
        MainPanel::Info => rsx! { InfoWindow { } },
    };

    cx.render(rsx! {
        div {
            class: "h-screen flex bg-gray-900 roboto-mono text-gray-300",
            tabindex: "-1",
            onkeydown: on_key_down,
            onkeypress: on_key_press,
            onkeyup: on_key_up,

            div { class: "md:basis-1/4"}
            div { class: "basis-1/2 h-screen flex flex-col mx-auto",
                Header { }
                panel
                Footer { }
            }
            div { class: "md:basis-1/4"}
        }
    })
}

fn Footer(cx: Scope) -> Element {
    let version = "v".to_owned() + env!("CARGO_PKG_VERSION");

    cx.render(rsx!(
        div {
            class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div {
                class: "flex flex-row gap-5 underline",
                a { href: "https://github.com/lectromoe/HemiTyper", "GitHub"}
                a { href: "mailto:contact@lectro.moe", "Feedback"}
            }
            div { " " }
            div {
                class: "flex flex-row gap-5",
                p { version }
            }
        }
    ))
}

fn Header(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let dict = use_context::<AppDictionaries>(&cx)?;

    let flip_side = move |_| {
        let side = app.read().side;
        match side {
            TypingSide::Left => {
                app.write().side = TypingSide::Right;
                app.write().keyboard = KeyboardState::new(&dict.read().right);
            }
            TypingSide::Right => {
                app.write().side = TypingSide::Left;
                app.write().keyboard = KeyboardState::new(&dict.read().left);
            }
        };
        app.write().typer.drain();
    };

    let toggle_info = move |_| {
        let panel = &mut app.write().panel;
        *panel = match panel {
            MainPanel::Typing => MainPanel::Info,
            MainPanel::Info => MainPanel::Typing,
        }
    };

    let toggle_sound = move |_| {
        let sound = &mut app.write().settings.sound_enabled;
        *sound = !*sound;
    };

    let toggle_keyboard = move |_| {
        let keyboard = &mut app.write().settings.keyboard_enabled;
        *keyboard = !*keyboard;
    };

    let app = app.read();
    let sound_enabled = app.settings.sound_enabled;
    let keyboard_enabled = app.settings.keyboard_enabled;

    cx.render(rsx!(
        div {
            class: "flex flex-row justify-between items-center m-5",
            div {
                a {
                    href: "#",
                    h1 {
                        class: "text-3xl md:text-4xl font-semibold tracking-tight leading-none text-gray-100",
                        mark { class: "px-2 mx-1 text-gray-100 bg-gray-700 rounded dark:bg-gray-700", "Hemi" }
                        "Typer"
                    }
                }
            }
            div { " " }
            div {
                class: "flex flex-row",
                ToggleButton { onclick: toggle_info, icon: Shape::InformationCircle }
                if keyboard_enabled {
                    rsx! { ToggleButton { onclick: toggle_keyboard, icon: Shape::Menu } }
                } else {
                    rsx! { ToggleButton { onclick: toggle_keyboard, icon: Shape::Minus } }
                }
                if sound_enabled {
                    rsx! { ToggleButton { onclick: toggle_sound, icon: Shape::VolumeUp } }
                } else {
                    rsx! { ToggleButton { onclick: toggle_sound, icon: Shape::VolumeOff } }
                }
                ToggleButton { onclick: flip_side, icon: Shape::Refresh }
            }
        }
    ))
}

#[inline_props]
fn ToggleButton<'a, S>(cx: Scope<S>, onclick: EventHandler<'a, MouseEvent>, icon: S) -> Element
where
    S: IconShape,
{
    cx.render(rsx!(
        a {
            class: "mt-3 ml-5",
            href: "#",
            onclick: move |evt| { onclick.call(evt); },
            Icon {
                class: "",
                fill: "white",
                size: 24,
                icon: icon.clone(),
            },
        },
    ))
}

fn TypingWindow(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let app = app.read();
    let keyboard_enabled = app.settings.keyboard_enabled;
    let status_enabled = app.settings.status_enabled;

    let next = app.typer.next_word().unwrap_or(" ");
    let prev = app.typer.last_word();
    let current = app.typer.input();

    let side_text_style = "pb-5 text-4xl font-bold text-transparent bg-clip-text 
                                bg-gradient-to-br from-teal-50 to-teal-200 basis-1/4";
    let main_text_style = "pb-5 text-4xl font-bold text-transparent bg-clip-text 
                                bg-gradient-to-br from-sky-300 to-sky-200 basis-1/4";

    let typing_panel = rsx! {
        div { class: "flex flex-row justify-center items-center content-center gap-5 p-10 my-auto h-32",
            h2 { class: "{side_text_style} text-right",  "{prev}" }
            h1 { class: "{main_text_style} text-center", "{current}" }
            h2 { class: "{side_text_style} text-left",   "{next}" }
        }
    };

    let status_bar = if status_enabled {
        rsx! { StatusBar { } }
    } else {
        rsx! { div { } }
    };

    let keyboard = if keyboard_enabled {
        rsx! { Keyboard { } }
    } else {
        rsx! { div { } }
    };

    cx.render(rsx! {
        div { class: "flex flex-col place-items-stretch h-screen gap-5 p-10",
            status_bar
            typing_panel
            keyboard
        }
    })
}

fn InfoWindow(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "flex flex-col justify-center items-center content-center gap-5 p-10 my-auto",
            div {
                class: "w-1/2 text-center",
                h1 { class: "text-xl tracking-tight text-white font-bold", "what" }
                p { class: "text-left",
                    "HemiTyper is an experimental typing trainer that helps to train
                    typing speed of your hands separately, providing you with only half 
                    the keyboard per training session."
                }
            }

            div {
                class: "w-1/2 text-center mt-5",
                h1 { class: "text-xl tracking-tight text-white font-bold", "why" }
                p { class: "text-left",
                    "I've found that training raw typing speed this way yields
                    great results long-term, but there wasn't many typing tutors that 
                    offer this kind of training - so I made one." 
                }
            }

            div {
                class: "w-1/2 text-center mt-5",
                h1 { class: "text-xl tracking-tight text-white font-bold", "next" }
                p { class: "text-left",
                    "After you're done training here, I highly recommend you
                    to continue with a full-featured typing trainer like "
                    span {
                        class: "underline",
                        a { class: "", href: "https://monkeytype.com/", "monkeytype"}
                    }
                    ", which this tool was heavily inspired by."
                }
            }

            div {
                class: "mt-20 text-center",
                "made with ❤ by ",
                span {
                    class: "underline",
                    a { class: "", href: "https://lectro.moe/", "lectro.moe"}
                }
            }
        }
    ))
}

fn StatusBar(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let wpm = 1;
    let streak = app.read().typer.streak();

    cx.render(rsx! {
        div { class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div {
                class: "flex flex-row",
                p { "WPM: {wpm}" }
            }
            div { " " }
            div { class: "flex flex-row gap-5",
                p { "Streak {streak}" }
            }
        }
    })
}

fn Keyboard(cx: Scope) -> Element {
    let app = use_context::<AppState>(&cx)?;
    let keyboard_state = &app.read().keyboard;

    let button_active = "w-16 h-14 text-gray-400 bg-white border-2 border-gray-300 
    focus:outline-none focus:ring-4 focus:ring-gray-200 
    font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-gray-800 
    text-white border-gray-600";

    let button_inactive = "w-16 h-14 text-gray-400 bg-white focus:outline-none focus:ring-4 
    focus:ring-gray-700 font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-gray-800 
    text-white border-gray-600";

    let keyboard = rsx! { keyboard_state.keys().iter().enumerate().map(|(i, row)| {
            let row_indent = (i * 10).to_string();
            rsx! {
                span { class: "ml-{row_indent}" }
                span {
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
                }
                br { }
            }
        }
    )};

    cx.render(rsx! {
        div {
            class: "content-center text-center overflow-visible w-max m-auto gap-5",
            id: "keyboard",
            keyboard
        }
    })
}

fn main() {
    // dioxus_tui::launch(App);
    dioxus_web::launch(App);
}
