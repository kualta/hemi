use eframe::egui::{self, CentralPanel, Style, Visuals};
use eframe::egui::epaint::Shadow;
use eframe::egui::{
    Align, Align2, Button, Color32, Context, InputState, RichText, Stroke, Ui, Vec2,
};
use eframe::Frame;
use eframe::epaint::Rounding;
use rand::Rng;
use std::default::Default;
use std::sync::Arc;
use std::vec::Vec;

pub struct TextContainer {
    keys: String,
    input_buffer: String,
    words_buffer: Vec<String>,
    current_index: usize,
    max_buffered_chars: u32,
}

impl TextContainer {
    fn new(keys: &str) -> TextContainer {
        TextContainer {
            input_buffer: "".to_owned(),
            words_buffer: Default::default(),
            current_index: 0,
            max_buffered_chars: 10,
            keys: keys.to_owned(),
        }
    }

    fn get_last_word(&self) -> Option<&String> {
        if self.current_index == 0 {
            return None;
        }
        return Some(&self.words_buffer[self.current_index]);
    }

    fn get_next_word(&self) -> Option<&String> {
        if self.current_index + 1 >= self.words_buffer.len() {
            return None;
        }
        return Some(&self.words_buffer[self.current_index + 1]);
    }

    fn generate_words(&mut self, amount: u32) {
        let mut rng = rand::thread_rng();
        let clean_char_set: Vec<char> = self.keys.clone().replace(" ", "").chars().collect();

        for _ in 0..amount {
            let mut new_word: String = "".to_owned();
            let max_length: u32 = rng.gen_range(3..=7);

            for _ in 1..max_length {
                let index = rng.gen_range(0..clean_char_set.len()) as usize;
                new_word.push(clean_char_set[index]);
            }
            self.words_buffer.push(new_word.to_string());
        }
    }

    fn update_input_buffer(&mut self, keyboard: &KeyboardState) {
        for row in &keyboard.rows {
            for key in row {
                if key.pressed {
                    self.input_buffer.push(key.character); // FIXME: just going through keys is not always correct
                }
                if key.key == egui::Key::Space && key.pressed {
                    self.input_buffer.clear();
                    self.try_increment();
                }
            }
        }

        if self.input_buffer.len() > self.max_buffered_chars as usize {
            self.input_buffer.remove(0);
        }
    }

    fn try_increment(&mut self) {
        if self.current_index + 1 >= self.words_buffer.len() {
            self.generate_words(32);
        } else {
            self.current_index += 1;
        }
    }
}

pub struct KeyboardState {
    rows: Vec<Vec<InputKey>>,
}

impl KeyboardState {
    fn new(keys: &str) -> Self {
        let mut keyboard = KeyboardState { rows: Vec::new() };

        for row in keys.split_whitespace() {
            let mut input_row = Vec::new();
            for c in row.chars() {
                let key = char_to_key(c);
                input_row.push(InputKey::new(c, key, false));
            }
            keyboard.rows.push(input_row);
        }

        // Add space bar as last input row
        let mut space_bar: Vec<InputKey> = Vec::new();
        space_bar.push(InputKey::new(
            ' ',
            egui::Key::Space,
            false
        ));
        keyboard.rows.push(space_bar);

        return keyboard;
    }
}

pub struct InputKey {
    character: char,
    key: egui::Key,
    pressed: bool,
}

impl InputKey {
    fn new(character: char, key: egui::Key, pressed: bool) -> Self {
        InputKey {
            character,
            key,
            pressed,
        }
    }
}

pub struct StyleConfig {
    button_size: f32,
    button_indent: f32,
    button_spacing: Vec2,
    window_shadow: Shadow,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            button_size: 75.,
            button_indent: 35.,
            button_spacing: Vec2::new(10., 10.),
            window_shadow: Shadow {
                extrusion: 0.,
                color: Color32::BLACK,
            },
        }
    }
}

pub struct TypingPanel {
    text: TextContainer,
    style: StyleConfig,
    keyboard: KeyboardState,
    title: String,
    align: Align2,
    enabled: bool, // FIXME: make an enum instead
}

impl TypingPanel {
    fn new(keys: &str) -> Self {
        TypingPanel {
                text: TextContainer::new(keys),
                style: Default::default(),
                keyboard: KeyboardState::new(keys),
                title: "left_panel".to_string(),
                align: Align2::LEFT_CENTER,
                enabled: true,
        }
    }

    fn update(&mut self, ctx: &Context) {
        if !self.enabled {
            return;
        }

        self.update_keyboard_state(&ctx.input());
        self.text.update_input_buffer(&self.keyboard);
    }

    fn update_keyboard_state(&mut self, input: &InputState) {
        for row in &mut self.keyboard.rows {
            for key in row {
                key.pressed = input.key_pressed(key.key);
            }
        }
    }

    fn draw(&mut self, ctx: &Context) {
        egui::Window::new(&self.title)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(self.align, Vec2::new(0., 0.))
            .min_height(800.)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top(&self.title)
                    .resizable(false)
                    .height_range(250. ..=250.)
                    .show_inside(ui, |ui| {
                        ui.add_space(125.);
                        ui.horizontal(|ui| {
                            ui.add_space(75.);
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text
                                        .get_last_word()
                                        .unwrap_or(&"".to_owned()),
                                )),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(&self.text.input_buffer)),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text
                                        .get_next_word()
                                        .unwrap_or(&"".to_owned()),
                                )),
                            );
                        });
                    });
                ui.add_space(120.);
                App::draw_keys(&self.style, ui, &mut self.keyboard);
                ui.add_space(50.);
            });
    }
}

pub struct ApplicationConfig {
    style: StyleConfig,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        ApplicationConfig {
            style: StyleConfig::default(),
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub config: ApplicationConfig,
    exit_requested: bool,
    left_panel: TypingPanel,
    right_panel: TypingPanel,
}

impl Default for App {
    fn default() -> Self {
        const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
        const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

        let mut app = App {
            config: ApplicationConfig::new(),
            exit_requested: false,
            left_panel: TypingPanel::new(LEFT_QWERTY_KEYS),
            right_panel: TypingPanel::new(RIGHT_QWERTY_KEYS),
        };

        app.right_panel.text.generate_words(10); // FIXME: Move to TypingPanel constructor
        app.left_panel.text.generate_words(10);

        app
    }
}

impl App {
    fn draw_keys(style_config: &StyleConfig, ui: &mut Ui, keyboard: &KeyboardState) {
        let button_size = style_config.button_size;
        ui.spacing_mut().item_spacing = style_config.button_spacing;

        let mut current_row_indent = 0.;
        for row in &keyboard.rows {
            ui.horizontal(|ui| {
                ui.add_space(current_row_indent);
                for key in row {
                    let width_mul = if key.key == egui::Key::Space { 4.6 } else { 1. };
                    ui.add_sized(
                        Vec2::new(button_size * width_mul, button_size),
                        Button::new(key.character.to_string())
                            //                   converting bool to either 0. or 1.
                            .stroke(Stroke::new(key.pressed as i32 as f32, Color32::WHITE)),
                    );
                }
            });
            current_row_indent += style_config.button_indent;
        }
    }

    fn draw_about_window(ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            egui::Area::new("about_area")
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.horizontal(|ui| {
                        egui::Layout::top_down_justified(Align::Center);
                        ui.label("made by ");
                        ui.hyperlink_to("lectroMathew", "https://github.com/lectroMathew");
                    });
                    ui.horizontal(|ui| {
                        ui.label("powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to(
                            "eframe",
                            "https://github.com/emilk/egui/tree/master/eframe",
                        );
                    });
                    ui.add_space(4.);
                    egui::warn_if_debug_build(ui);
                });
        });
    }

    fn draw_top_bar(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            let mut resize_requested = false;

            ui.horizontal(|ui| {
                if ui
                    .checkbox(&mut self.left_panel.enabled, "Left panel")
                    .changed()
                {
                    resize_requested = true;
                }
                if ui
                    .checkbox(&mut self.right_panel.enabled, "Right panel")
                    .changed()
                {
                    resize_requested = true;
                }
            });

            if resize_requested {
                let mut new_size = Vec2::new(500., 800.);

                if self.right_panel.enabled && self.left_panel.enabled {
                    new_size.x += 500.0;
                }

                frame.set_window_size(new_size);
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.draw_top_bar(ctx, frame);

        CentralPanel::default().show(ctx, |_ui| {
            self.left_panel.update(ctx);
            self.right_panel.update(ctx);

            self.left_panel.draw(ctx);
            self.right_panel.draw(ctx);

            if !self.left_panel.enabled && !self.right_panel.enabled {
                App::draw_about_window(ctx);
            }
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}

fn char_to_key(c: char) -> egui::Key {
    match c {
        'A' => egui::Key::A,
        'B' => egui::Key::B,
        'C' => egui::Key::C,
        'D' => egui::Key::D,
        'E' => egui::Key::E,
        'F' => egui::Key::F,
        'G' => egui::Key::G,
        'H' => egui::Key::H,
        'I' => egui::Key::I,
        'J' => egui::Key::J,
        'K' => egui::Key::K,
        'L' => egui::Key::L,
        'M' => egui::Key::M,
        'N' => egui::Key::N,
        'O' => egui::Key::O,
        'P' => egui::Key::P,
        'Q' => egui::Key::Q,
        'R' => egui::Key::R,
        'S' => egui::Key::S,
        'T' => egui::Key::T,
        'U' => egui::Key::U,
        'V' => egui::Key::V,
        'W' => egui::Key::W,
        'X' => egui::Key::X,
        'Y' => egui::Key::Y,
        'Z' => egui::Key::Z,
        ' ' => egui::Key::Space,
        // ';' => egui::Key::Semicolon,
        // TODO: Add special characters handling when egui adds support for them ¯\_(ツ)_/¯
        _ => egui::Key::Escape,
    }
}