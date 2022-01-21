use std::alloc::Layout;
use std::collections::hash_map::IntoKeys;
use std::sync::Arc;
use std::vec::Vec;
use std::default::Default;
use std::ops::Div;
use eframe::{egui, epi};
use eframe::egui::{Align, Align2, Button, CentralPanel, Color32, Context, CtxRef, Pos2, Rgba, RichText, Stroke, Style, TextBuffer, TextStyle, Ui, Vec2, Visuals, Window};
use eframe::egui::epaint::Shadow;
use eframe::egui::Event::Key;
use eframe::egui::WidgetType::ColorButton;
use eframe::epi::Frame;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";
const QWERTY_KEYS: &str = "QWERTYUIOP ASDFGHJKL\' ZXCVBNM,./";

pub struct TextContainer {
    input_buffer: String,
    generated_buffer: Vec<String>,
    current_index: usize,
    max_buffered_chars: u32,
}

impl Default for TextContainer {
    fn default() -> Self {
        TextContainer {
            input_buffer: "".to_owned(),
            generated_buffer: Default::default(),
            current_index: 0,
            max_buffered_chars: 10
        }
    }
}

impl TextContainer {
    fn get_last_word(&self) -> Option<&String> {
        if self.current_index == 0 {
            return None;
        }
        return Some(&self.generated_buffer[self.current_index - 1])
    }

    fn get_next_word(&self) -> Option<&String> {
        if self.current_index + 1 >= self.generated_buffer.len() {
            return None;
        }
        return Some(&self.generated_buffer[self.current_index + 1])
    }

    fn generate_words(&mut self, amount: u32, char_set: &str) {
        let mut rng = rand::thread_rng();

        for _ in 0..amount {
            for length in 4..5 {

            }
        }
    }

    fn update_input_buffer(&mut self, input_state: &Vec<Vec<InputKey>>) {
        for row in input_state {
            for key in row {
                if key.pressed {
                    self.input_buffer.push(key.character);
                }
                if key.key == egui::Key::Space && key.pressed{
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
        if self.current_index + 1 >= self.generated_buffer.len() {
            self.generate_words(32);
        } else {
            self.current_index += 1;
        }
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
                color: Color32::BLACK
            }
        }
    }
}

pub struct TypingPanel {
    text_container: TextContainer,
    style_config: StyleConfig,
    title: String,
    char_set: String,
    align: Align2,
    enabled: bool,
}

impl TypingPanel {
    fn update_and_draw(&mut self, ctx: &CtxRef) {
        if !self.enabled { return; }

        let input_state = App::update_key_state(ctx, &self.char_set);

        self.text_container.update_input_buffer(&input_state);
        self.draw(ctx, &input_state);
    }

    fn draw(&self, ctx: &CtxRef, input_state: &Vec<Vec<InputKey>>) {
        egui::Window::new(&self.title)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(self.align, Vec2::new(0., 0.))
            .min_height(800.)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top(&self.title)
                    .resizable(false)
                    .height_range(250. ..= 250.)
                    .show_inside(ui, |ui| {
                        ui.add_space(125.);
                        ui.horizontal(|ui| {
                            ui.add_space(150.);
                            ui.label(RichText::from(self.text_container
                                .get_last_word()
                                .unwrap_or(&"".to_owned())));
                            ui.add_sized(Vec2::new(100., 30.), egui::Label::new(
                                RichText::from(&self.text_container.input_buffer)));
                            ui.label(RichText::from(self.text_container
                                .get_next_word()
                                .unwrap_or(&"".to_owned())));
                        });
                    });
                ui.add_space(120.);
                App::draw_keys(&self.style_config, ui, &input_state);
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

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
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
        Self {
            config: ApplicationConfig::new(),
            exit_requested: false,
            left_panel: TypingPanel {
                text_container: Default::default(),
                style_config: Default::default(),
                title: "left_panel".to_string(),
                char_set: LEFT_QWERTY_KEYS.to_string(),
                align: Align2::LEFT_CENTER,
                enabled: true
            },
            right_panel: TypingPanel {
                text_container: Default::default(),
                style_config: Default::default(),
                title: "right_panel".to_string(),
                char_set: RIGHT_QWERTY_KEYS.to_string(),
                align: Align2::RIGHT_CENTER,
                enabled: true
            },
        }
    }
}

impl App {
    pub fn exit(&mut self) {
        self.exit_requested = true;
    }

    pub fn update_key_state(ctx: &CtxRef, keys: &str) -> Vec<Vec<InputKey>> {
        let mut input_state: Vec<Vec<InputKey>> = Vec::new();
        for row in keys.split_whitespace() {
            let mut input_row = Vec::new();
            for c in row.chars() {
                let key = char_to_key(c);
                input_row.push(InputKey::new(c, key, ctx.input().key_pressed(key)));
            }
            input_state.push(input_row);
        }

        // Add space bar as last input row
        let mut space_bar: Vec<InputKey> = Vec::new();
        space_bar.push(InputKey::new(' ', egui::Key::Space,
                                     ctx.input().key_pressed(egui::Key::Space)));
        input_state.push(space_bar);

        return input_state
    }

    fn draw_keys(style_config: &StyleConfig, ui: &mut Ui, input_state: &Vec<Vec<InputKey>>) {
        let button_size = style_config.button_size;
        ui.spacing_mut().item_spacing = style_config.button_spacing;

        let mut current_row_indent = 0.;
        for row in input_state {
            ui.horizontal(|ui| {
                ui.add_space(current_row_indent);
                for key in row {
                    let width_mul = if key.key == egui::Key::Space { 4.6 } else { 1. };
                    ui.add_sized(Vec2::new(button_size * width_mul, button_size), Button::new(key.character.to_string())

                            //                   converting bool to either 0. or 1.
                            .stroke(Stroke::new(key.pressed as i32 as f32, Color32::WHITE))
                    );
                }
            });
            current_row_indent += style_config.button_indent;
        };
    }

    fn draw_about_window(ctx: &CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                            ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                        });
                        ui.add_space(4.);
                        egui::warn_if_debug_build(ui);
                    });
        });
    }

    fn draw_top_bar(&mut self, ctx: &CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            let mut resize_requested = false;

            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.left_panel.enabled, "Left panel").changed() {
                    resize_requested = true;
                }
                if ui.checkbox(&mut self.right_panel.enabled, "Right panel").changed() {
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

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        self.draw_top_bar(ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {

            self.left_panel.update_and_draw(ctx);
            self.right_panel.update_and_draw(ctx);

            if !self.left_panel.enabled && !self.right_panel.enabled {
                App::draw_about_window(ctx);
            }
        });

        if self.exit_requested { frame.quit() };
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        let app_style = Style {
            body_text_style: TextStyle::Small,
            override_text_style: None,
            wrap: None,
            spacing: Default::default(),
            interaction: Default::default(),
            visuals: Visuals {
                window_corner_radius: 0.0,
                window_shadow: self.config.style.window_shadow,
                ..Default::default()
            },
            animation_time: 0.0,
            debug: Default::default(),
            explanation_tooltips: false
        };

        _ctx.set_style(Arc::new(app_style));
        self.right_panel.text_container.generate_words(10);
        self.left_panel.text_container.generate_words(10);
    }

    /// Saves the state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "HemiTyper"
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
        _ => egui::Key::Escape
    }
}

