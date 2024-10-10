#![allow(non_snake_case)]

mod words;

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
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(AudioLibrary::default()));
    let mut audio = use_context::<Signal<AudioLibrary>>();

    use_context_provider(|| Signal::new(LayoutDictionary::default()));
    let mut dictionary = use_context::<Signal<LayoutDictionary>>();

    use_context_provider(|| Signal::new(AppState::new(&dictionary.read().left)));
    let mut app = use_context::<Signal<AppState>>();

    use_context_provider(|| Signal::new(Layouts::default()));
    let mut layouts_state = use_context::<Signal<Layouts>>();

    let layouts = use_resource(|| async move { Layouts::pull().await });
    let mut init = use_signal(|| false);

    if let Some(ref data) = *layouts.read() {
        if !*init.read() {
            let mut app = app.write();
            *layouts_state.write() = data.clone();

            *dictionary.write() = match app.layout {
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
            Code::Space | Code::Enter => {
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

    rsx! {
        div {
            class: "h-screen flex bg-black roboto-mono text-white",
            tabindex: "-1",
            onkeydown: on_key_down,
            onkeypress: on_key_press,
            onkeyup: on_key_up,
            div { class: "md:basis-1/4" },
            div { class: "basis-1/2 h-screen flex flex-col mx-auto",
                Header {},
                {panel},
                Footer {}
            },
            div { class: "md:basis-1/4" }
        }
    }
}

fn Footer() -> Element {
    let mut app = use_context::<Signal<AppState>>();
    let version = "v".to_owned() + env!("CARGO_PKG_VERSION");

    let toggle_info = move |_| {
        let panel = &mut app.write().panel;

        *panel = match panel {
            MainPanel::Typing => MainPanel::Info,
            MainPanel::Info => MainPanel::Typing,
        };
    };

    rsx! {
        div { class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div { class: "flex flex-row gap-3",
                a { class: "underline", href: "https://github.com/kualta/Hemi", "source" }
                button { class: "underline", onclick: toggle_info, "about" }
            }
            div { " " }
            div { class: "flex flex-row gap-5",
                p { "{version}" }
            }
        }
    }
}

fn Header() -> Element {
    let mut app = use_context::<Signal<AppState>>();
    let mut dictionary = use_context::<Signal<LayoutDictionary>>();
    let layouts = use_context::<Signal<Layouts>>();

    let flip_side = move |_| {
        let mut app = app.write();

        let newSide = match app.side {
            TypingSide::Left => TypingSide::Right,
            TypingSide::Right => TypingSide::Left,
        };

        app.side = newSide;
        app.typer.drain();
        app.refresh_keyboard(&dictionary.read());
    };

    let switch_layout = move |e: Event<FormData>| {
        let mut app = app.write();
        let mut dictionary = dictionary.write();
        let layouts = layouts.read();
        let current_layout = &mut app.layout;

        *current_layout = match e.value().as_str() {
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

    rsx! {
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
                    ToggleButton { onclick: toggle_keyboard, icon: "keyboard" }
                } else {
                    ToggleButton { onclick: toggle_keyboard, icon: "keyboard_off" }
                }
                if sound_enabled {
                    ToggleButton { onclick: toggle_sound, icon: "volume_up" }
                } else {
                    ToggleButton { onclick: toggle_sound, icon: "volume_off" }
                }
                ToggleButton { onclick: flip_side, icon: "cached" },
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
}

#[component]
fn ToggleButton(onclick: EventHandler<MouseEvent>, icon: String) -> Element {
    rsx! {
        a {
            class: "mt-3 ml-5",
            href: "#",
            onclick: move |evt| onclick.call(evt),
            {
                match icon.as_str() {
                    "keyboard" => icons::keyboard(),
                    "keyboard_off" => icons::keyboard_off(),
                    "volume_up" => icons::volume_up(),
                    "volume_off" => icons::volume_off(),
                    "cached" => icons::cached(),
                    _ => rsx!(""),
                }
            }
        }
    }
}

fn TypingWindow() -> Element {
    let app = use_context::<Signal<AppState>>();
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
            h2 { class: "{side_text_style}", "{prev}" }
            h1 { class: "{main_text_style}", "{current}" }
            h2 { class: "{side_text_style}", "{next}" }
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

    rsx! {
        div { class: "flex flex-col place-items-stretch h-screen gap-5 p-10", {status_bar}, {typing_panel}, {keyboard} }
    }
}

fn InfoWindow() -> Element {
    rsx!(
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
                " and community"
            }
        }
    )
}

fn StatusBar() -> Element {
    let app = use_context::<Signal<AppState>>();
    let streak = app.read().typer.streak();

    rsx! {
        div { class: "flex flex-row justify-between items-center m-5 text-sm text-neutral-400",
            div { class: "flex flex-row gap-5", p { "streak: {streak}" } }
        }
    }
}

fn Keyboard() -> Element {
    let app = use_context::<Signal<AppState>>();
    let keyboard = &app.read().keyboard;

    let button_active = "w-16 h-14 text-white border-2 border-zinc-300 
    focus:outline-none focus:ring-4 focus:ring-zinc-200
     font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-[#27272a]
      text-white border-zinc-600";
    let button_inactive = "w-16 h-14 text-zinc-400 focus:outline-none focus:ring-4
     focus:ring-zinc-700 font-medium rounded-lg text-xl px-5 py-2.5 mr-2 mb-2 bg-[#27272a]
      text-white border-zinc-600";

    let keyboard = rsx! {
        {keyboard.keys().iter().enumerate().map(|(i, row)| {
            let row_indents = ["ml-10", "ml-20", "ml-[7.5rem]"];
            rsx! {
                span { class: row_indents[i] }
                span {
                    {row.iter().map(|key| {
                        let button_style = if key.enabled() { button_active } else { button_inactive };
                        rsx! {
                            button {
                                class: "{button_style}",
                                 "type": "button",
                                  "{key.key()}"
                                 }
                        }
                    })}
                }
                br { }
            }
        })}
    };

    rsx! {
        div {
            class: "content-center text-center overflow-visible w-max m-auto gap-5",
            id: "keyboard",
            {keyboard}
        }
    }
}

mod icons {
    use super::*;

    pub(super) fn keyboard() -> Element {
        rsx! {
            svg { class: "mr-3",
                fill: "#FFFFFF",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 -960 960 960",
                width: "24",
                height: "24",
                path {
                    stroke_linejoin: "round",
                    d: "M160-200q-33 0-56.5-23.5T80-280v-400q0-33 23.5-56.5T160-760h640q33 0 56.5 23.5T880-680v400q0 33-23.5 56.5T800-200H160Zm0-80h640v-400H160v400Zm160-40h320v-80H320v80ZM200-440h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM200-560h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80Zm120 0h80v-80h-80v80ZM160-280v-400 400Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(super) fn keyboard_off() -> Element {
        rsx! {
            svg { class: "mr-3",
                fill: "#FFFFFF",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 -960 960 960",
                width: "24",
                height: "24",
                path {
                    stroke_linejoin: "round",
                    d: "M794-59 59-794l57-57 735 735-57 57ZM320-320v-80h247l80 80H320ZM200-440v-80h80v80h-80Zm120 0v-80h80v80h-80Zm360 0v-80h80v80h-80ZM200-560v-80h80v80h-80Zm360 0v-80h80v80h-80Zm120 0v-80h80v80h-80Zm176 337-56-57v-400H401l-80-80h479q33 0 56.5 23.5T880-680v401q0 17-6.5 31T856-223Zm-696 23q-33 0-56.5-23.5T80-280v-400q0-33 23.5-56.5T160-760h47l80 80H160v400h527l80 80H160Zm281-440h79v79l-79-79Zm119 120h80v79l-80-79Zm-187 40Zm227 0Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(super) fn volume_up() -> Element {
        rsx! {
            svg { class: "mr-3",
                fill: "#FFFFFF",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 -960 960 960",
                width: "24",
                height: "24",
                path {
                    stroke_linejoin: "round",
                    d: "M560-131v-82q90-26 145-100t55-168q0-94-55-168T560-749v-82q124 28 202 125.5T840-481q0 127-78 224.5T560-131ZM120-360v-240h160l200-200v640L280-360H120Zm440 40v-322q47 22 73.5 66t26.5 96q0 51-26.5 94.5T560-320ZM400-606l-86 86H200v80h114l86 86v-252ZM300-480Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(super) fn volume_off() -> Element {
        rsx! {
            svg { class: "mr-3",
                fill: "#FFFFFF",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 -960 960 960",
                width: "24",
                height: "24",
                path {
                    stroke_linejoin: "round",
                    d: "M792-56 671-177q-25 16-53 27.5T560-131v-82q14-5 27.5-10t25.5-12L480-368v208L280-360H120v-240h128L56-792l56-56 736 736-56 56Zm-8-232-58-58q17-31 25.5-65t8.5-70q0-94-55-168T560-749v-82q124 28 202 125.5T840-481q0 53-14.5 102T784-288ZM650-422l-90-90v-130q47 22 73.5 66t26.5 96q0 15-2.5 29.5T650-422ZM480-592 376-696l104-104v208Zm-80 238v-94l-72-72H200v80h114l86 86Zm-36-130Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
        }
    }

    pub(super) fn cached() -> Element {
        rsx! {
            svg { class: "mr-3",
                fill: "#FFFFFF",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 -960 960 960",
                width: "24",
                height: "24",
                path {
                    stroke_linejoin: "round",
                    d: "M482-160q-134 0-228-93t-94-227v-7l-64 64-56-56 160-160 160 160-56 56-64-64v7q0 100 70.5 170T482-240q26 0 51-6t49-18l60 60q-38 22-78 33t-82 11Zm278-161L600-481l56-56 64 64v-7q0-100-70.5-170T478-720q-26 0-51 6t-49 18l-60-60q38-22 78-33t82-11q134 0 228 93t94 227v7l64-64 56 56-160 160Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
        }
    }
}
