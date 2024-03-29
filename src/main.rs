#![allow(non_snake_case)]

mod words;

use dioxus_material_symbols::{MaterialIcon, MaterialIconStylesheet};
use dioxus::events::{KeyboardData, MouseEvent};
use dioxus::html::input_data::keyboard_types::{Code, Key};
use dioxus::prelude::*;
use words::*;

#[derive(Clone, Copy)]
enum MainPanel {
    Typing,
    Info,
}

#[derive(Clone, Debug, Copy)]
enum TypingSide {
    Left,
    Right,
}

#[derive(Clone, Debug, Copy)]
enum KeyboardLayout {
    Qwerty,
    Dvorak,
    Colemak,
    Workman,
    Custom,
}

#[derive(Clone)]
pub(crate) struct AppSettings {
    sound_enabled: bool,
    status_enabled: bool,
    keyboard_enabled: bool,
}

#[derive(Clone)]
pub(crate) struct AppState {
    keyboard: KeyboardState,
    settings: AppSettings,
    typer: TypingData,
    layout: KeyboardLayout,
    panel: MainPanel,
    side: TypingSide,
}

impl AppState {
    pub(crate) fn refresh_keyboard(&mut self, dictionary: &LayoutDictionary) {
        match self.side {
            TypingSide::Left => {
                self.keyboard = KeyboardState::new(&dictionary.left);
            }
            TypingSide::Right => {
                self.keyboard = KeyboardState::new(&dictionary.right);
            }
        };
    }

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
            layout: KeyboardLayout::Qwerty,
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, AudioLibrary::default);
    let audio = use_shared_state::<AudioLibrary>(cx)?;

    use_shared_state_provider(cx, LayoutDictionary::default);
    let dictionary = use_shared_state::<LayoutDictionary>(cx)?;

    use_shared_state_provider(cx, || AppState::new(&dictionary.read().left));
    let app = use_shared_state::<AppState>(cx)?;

    use_shared_state_provider(cx, || Layouts::default());
    let layouts_state = use_shared_state::<Layouts>(cx)?;

    let layouts = use_future(cx, (), |_| async move { Layouts::pull().await });
    let init = use_state(cx, || false);

    if let Some(data) = layouts.value() {
        if !init {
            let mut app = app.write_silent();
            *layouts_state.write_silent() = data.clone();

            *dictionary.write_silent() = match app.layout {
                KeyboardLayout::Qwerty => data.qwerty.clone(),
                KeyboardLayout::Colemak => data.colemak.clone(),
                _ => data.qwerty.clone(),
            };

            app.refresh_keyboard(&dictionary.read());
            app.typer.submit();

            init.set(true);
        }
    }

    let on_key_down = move |event: Event<KeyboardData>| {
        let key_code = event.code();

        match key_code {
            Code::Backspace => {
                app.write().typer.pop();
            }
            Code::Space => {
                app.write().typer.submit();
            }
            Code::Enter => {
                app.write().typer.submit();
            }
            _ => (),
        }

        if app.write().typer.buffer().is_empty() {
            let app_dict = dictionary.read();
            let mut app = app.write();
            let dictionary = match app.side {
                TypingSide::Left => &app_dict.left,
                TypingSide::Right => &app_dict.right,
            };
            app.typer.generate_words(10, dictionary);
        }

        if app.write().settings.sound_enabled {
            audio.write().play(key_code);
        }
    };

    let on_key_press = move |event: Event<KeyboardData>| {
        let key = &event.key();
        app.write().keyboard.update_for(&KeyState::new(key, true));

        if let Key::Character(key) = key {
            app.write().typer.push_str(&key.to_string());
        };
    };

    let on_key_up = move |event: Event<KeyboardData>| {
        let key = &event.key();
        app.write().keyboard.update_for(&KeyState::new(key, false));
    };

    let panel = match app.read().panel {
        MainPanel::Typing => rsx! { TypingWindow {} },
        MainPanel::Info => rsx! { InfoWindow {} },
    };

    cx.render(rsx! {
        div {
            class: "h-screen flex bg-black roboto-mono text-white",
            tabindex: "-1",
            onkeydown: on_key_down,
            onkeypress: on_key_press,
            onkeyup: on_key_up,

            MaterialIconStylesheet {}
            div { class: "md:basis-1/4" }
            div { class: "basis-1/2 h-screen flex flex-col mx-auto",
                Header {}
                panel,
                Footer {}
            }
            div { class: "md:basis-1/4" }
        }
    })
}

fn Footer(cx: Scope) -> Element {
    let app = use_shared_state::<AppState>(cx)?;
    let version = "v".to_owned() + env!("CARGO_PKG_VERSION");

    let toggle_info = move |_| {
        let panel = &mut app.write().panel;

        *panel = match panel {
            MainPanel::Typing => MainPanel::Info,
            MainPanel::Info => MainPanel::Typing,
        };
    };

    cx.render(rsx!(
        div { class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div { class: "flex flex-row gap-3",
                a { class: "underline", href: "https://github.com/kualta/Hemi", "source" }
                button { class: "underline", onclick: toggle_info, "about" }
            }
            div { " " }
            div { class: "flex flex-row gap-5",
                p { version }
            }
        }
    ))
}

fn Header(cx: Scope) -> Element {
    let app = use_shared_state::<AppState>(cx)?;
    let dictionary = use_shared_state::<LayoutDictionary>(cx)?;
    let layouts = use_shared_state::<Layouts>(cx)?;

    let flip_side = move |_| {
        let mut app = app.write();

        match app.side {
            TypingSide::Left => {
                app.side = TypingSide::Right;
            }
            TypingSide::Right => {
                app.side = TypingSide::Left;
            }
        };

        app.typer.drain();
        app.refresh_keyboard(&dictionary.read());
    };

    let switch_layout = move |e: Event<FormData>| {
        let mut app = app.write();
        let mut dictionary = dictionary.write();
        let layouts = layouts.read();
        let current_layout = &mut app.layout;

        *current_layout = match e.value.as_str() {
            "colemak" => KeyboardLayout::Colemak,
            "workman" => KeyboardLayout::Workman,
            "qwerty" => KeyboardLayout::Qwerty,
            "dvorak" => KeyboardLayout::Dvorak,
            "custom" => KeyboardLayout::Custom,
            _ => KeyboardLayout::Qwerty,
        };

        *dictionary = match current_layout {
            KeyboardLayout::Qwerty => layouts.qwerty.clone(),
            KeyboardLayout::Colemak => layouts.colemak.clone(),
            _ => layouts.qwerty.clone(),
        };

        app.typer.drain();
        app.refresh_keyboard(&dictionary);
    };


    let toggle_sound = move |_| {
        let sound = &mut app.write().settings.sound_enabled;
        *sound = !*sound;
    };

    let toggle_keyboard = move |_| {
        let keyboard = &mut app.write().settings.keyboard_enabled;
        *keyboard = !*keyboard;
    };

    let sound_enabled = app.read().settings.sound_enabled;
    let keyboard_enabled = app.read().settings.keyboard_enabled;

    cx.render(rsx!(
        div { class: "flex flex-row justify-between items-center m-5",
            div {
                a { href: "#",
                    h1 { class: "text-3xl md:text-4xl font-semibold tracking-tight leading-none text-white",
                        mark { class: "px-2 mx-1 text-white bg-[#27272a] rounded dark:bg-[#27272a]",
                            "Hemi"
                        }
                        "Typer"
                    }
                }
            }
            div { " " }
            div { class: "flex flex-row",
                if keyboard_enabled {
                    rsx! { ToggleButton { onclick: toggle_keyboard, icon: "keyboard" } }
                } else {
                    rsx! { ToggleButton { onclick: toggle_keyboard, icon: "keyboard_off" } }
                }
                if sound_enabled {
                    rsx! { ToggleButton { onclick: toggle_sound, icon: "volume_up" } }
                } else {
                    rsx! { ToggleButton { onclick: toggle_sound, icon: "volume_off" } }
                }
                rsx! { ToggleButton { onclick: flip_side, icon: "cached" } },
                rsx! { 
                    select { class: "mt-2 ml-5 bg-transparent dark:bg-transparent border border-white text-sm rounded-lg appearance-none text-center p-1 px-1.5 pb-1.5 items-center justify-center",
                        name: "layout",
                        id: "layout",
                        onchange: switch_layout,
                        option { value: "qwerty", "qwerty" }
                        option { value: "colemak", "colemak" }
                    }
                }
            }
        }
    ))
}

#[component]
fn ToggleButton<'a>(cx: Scope, onclick: EventHandler<'a, MouseEvent>, icon: &'a str) -> Element {
    cx.render(rsx! {
        a { class: "mt-3 ml-5", href: "#", onclick: move |evt| {
                onclick.call(evt);
            },
            MaterialIcon { color: "white", name: icon, size: 24 }
        }
    })
}

fn TypingWindow(cx: Scope) -> Element {
    let app = use_shared_state::<AppState>(cx)?;
    let keyboard_enabled = app.read().settings.keyboard_enabled;
    let status_enabled = app.read().settings.status_enabled;

    let app = app.read();
    let next = app.typer.next_word().unwrap_or(" ");
    let prev = app.typer.last_word();
    let current = app.typer.input();

    let side_text_style = "pb-5 text-4xl font-bold text-transparent bg-clip-text 
                                bg-gradient-to-br from-zinc-50 to-zinc-200 basis-1/4";
    let main_text_style = "pb-5 text-4xl font-bold text-white basis-1/4";

    let typing_panel = rsx! {
        div { class: "flex flex-row justify-center items-center content-center gap-5 p-10 my-auto h-32",
            h2 { class: "{side_text_style} text-right", "{prev}" }
            h1 { class: "{main_text_style} text-center", "{current}" }
            h2 { class: "{side_text_style} text-left", "{next}" }
        }
    };

    let status_bar = if status_enabled {
        rsx! { StatusBar {} }
    } else {
        rsx! { div {} }
    };

    let keyboard = if keyboard_enabled {
        rsx! { Keyboard {} }
    } else {
        rsx! { div {} }
    };

    cx.render(rsx! {
        div { class: "flex flex-col place-items-stretch h-screen gap-5 p-10", status_bar, typing_panel, keyboard }
    })
}

fn InfoWindow(cx: Scope) -> Element {
    cx.render(rsx!(
        div { class: "flex flex-col justify-center items-center content-center gap-5 p-10 my-auto",
            div { class: "w-96 m-auto text-center",
                h1 { class: "text-xl tracking-tight text-white font-bold", "what" }
                p { class: "text-left",
                    "Hemi is a typing trainer that allows you to improve typing speed of your hands 
                    separately, providing you with only half the keyboard per training session."
                }
            }

            div { class: "w-96 m-auto text-center mt-5",
                h1 { class: "text-xl tracking-tight text-white font-bold", "why" }
                p { class: "text-left",
                    "I found that training raw typing speed this way yields
                    great results, but there wasn't any typing tutors that focused 
                    on this kind of training"
                }
            }

            div { class: "w-96 m-auto text-center mt-5",
                h1 { class: "text-xl tracking-tight text-white font-bold", "next" }
                p { class: "text-left",
                    "After you're done training here, I recommend you
                    to continue with a full-featured typing trainer like "
                    span { class: "underline", a { class: "", href: "https://monkeytype.com/", "monkeytype" } }
                    ", which this tool was heavily inspired by."
                }
            }

            div { class: "mt-20 text-center",
                "made with ❤ by "
                span { class: "underline", a { class: "", href: "https://kualta.dev/", "kualta" } }
            }
        }
    ))
}

fn StatusBar(cx: Scope) -> Element {
    let app = use_shared_state::<AppState>(cx)?;
    let app = app.read();
    let streak = app.typer.streak();

    cx.render(rsx! {
        div { class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div { class: "flex flex-row gap-5", p { "streak: {streak}" } }
        }
    })
}

fn Keyboard(cx: Scope) -> Element {
    let app = use_shared_state::<AppState>(cx)?;
    let keyboard = &app.read().keyboard;

    let button_active = "w-16 h-14 text-white border-2 border-zinc-300 
    focus:outline-none focus:ring-4 focus:ring-zinc-200 
    font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-[#27272a]
    text-white border-zinc-600";

    let button_inactive = "w-16 h-14 text-zinc-400 focus:outline-none focus:ring-4 
    focus:ring-zinc-700 font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-[#27272a] 
    text-white border-zinc-600";

    let keyboard = rsx! {
        keyboard.keys().iter().enumerate().map(|(i, row)| {
            let row_indents = ["ml-10", "ml-20", "ml-[7.5rem]"];
            rsx! {
                span { class: row_indents[i] }
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
        )
    };

    cx.render(rsx! {
        div {
            class: "content-center text-center overflow-visible w-max m-auto gap-5",
            id: "keyboard",
            keyboard
        }
    })
}
