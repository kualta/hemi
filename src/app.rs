use std::alloc::Layout;
use std::collections::hash_map::IntoKeys;
use std::sync::Arc;
use std::vec::Vec;
use eframe::{egui, epi};
use eframe::egui::{Align, Align2, Button, CentralPanel, Color32, Context, CtxRef, Pos2, Rgba, RichText, Stroke, Style, TextStyle, Ui, Vec2, Visuals, Window};
use eframe::egui::epaint::Shadow;
use eframe::egui::Event::Key;
use eframe::egui::WidgetType::ColorButton;
use eframe::epi::Frame;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

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
    keyboard_top_indent: f32,
    window_shadow: Shadow,
}

impl Default for StyleConfig {
    fn default() -> Self {
        StyleConfig {
            button_size: 75.,
            button_indent: 35.,
            button_spacing: Vec2::new(10., 10.),
            keyboard_top_indent: 400.,
            window_shadow: Shadow {
                extrusion: 0.1,
                color: Color32::RED
            }
        }
    }
}

pub struct ApplicationConfig {
    pub side_enabled: (bool, bool),
    pub style: StyleConfig,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        ApplicationConfig {
            side_enabled: (true, true),
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
    resize_requested: bool,
    left_text_buffer: String,
    right_text_buffer: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: ApplicationConfig::new(),
            exit_requested: false,
            resize_requested: false,
            left_text_buffer: Default::default(),
            right_text_buffer: Default::default()
        }
    }
}

impl App {
    pub fn exit(&mut self) {
        self.exit_requested = true;
    }

    fn recalculate_size(&mut self, frame: &Frame) {
        let mut new_size = Vec2::new(500., 800.);

        if self.config.side_enabled.0 && self.config.side_enabled.1 {
            new_size.x += 500.0;
        }

        frame.set_window_size(new_size);
    }

    fn draw_right_panel(&mut self, ctx: &CtxRef) {
        egui::Window::new("right_panel")
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(Align2::RIGHT_CENTER, Vec2::new(0., 0.))
            .min_height(800.)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("right_text_panel")
                    .resizable(false)
                    .height_range(300. ..= 300.)
                    .show_inside(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::from(&self.right_text_buffer));
                        })
                    });
                ui.add_space(150.);
                let input_state = self.check_pressed(ui, RIGHT_QWERTY_KEYS);
                App::update_buffer(&mut self.right_text_buffer, &input_state);
                self.draw_keys(ui, input_state);
                ui.add_space(50.);
            });
    }

    fn draw_left_panel(&mut self, ctx: &CtxRef) {
        egui::Window::new("left_panel")
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .anchor(Align2::LEFT_CENTER, Vec2::new(0., 0.))
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("left_text_panel")
                    .resizable(false)
                    .height_range(300. ..= 300.)
                    .show_inside(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(RichText::from(&self.left_text_buffer));
                        })
                    });
                ui.add_space(150.);
                let input_state = self.check_pressed(ui, LEFT_QWERTY_KEYS);
                App::update_buffer(&mut self.left_text_buffer, &input_state);
                self.draw_keys(ui, input_state);
                ui.add_space(50.);
            });
    }

    fn check_pressed(&mut self, ui: &mut Ui, keys: &str) -> Vec<Vec<InputKey>> {
        let mut input_state: Vec<Vec<InputKey>> = Vec::new();
        for row in keys.split_whitespace() {
            let mut input_row = Vec::new();
            for c in row.chars() {
                let key = char_to_key(c);
                input_row.push(InputKey::new(c, key, ui.input().key_pressed(key)));
            }
            input_state.push(input_row);
        }

        return input_state
    }

    fn draw_keys(&mut self, ui: &mut Ui, input_state: Vec<Vec<InputKey>>) {
        let button_size = self.config.style.button_size;
        ui.spacing_mut().item_spacing = self.config.style.button_spacing;

        let mut current_row_indent = 0.;
        for row in input_state {
            ui.horizontal(|ui| {
                ui.add_space(current_row_indent);
                for key in row {
                    ui.add_sized(Vec2::new(button_size, button_size), Button::new(key.character.to_string())

                            //                   converting bool to either 0. or 1.
                            .stroke(Stroke::new(key.pressed as i32 as f32, Color32::WHITE))
                    );
                }
            });
            current_row_indent += self.config.style.button_indent;
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

    fn draw_top_bar(&mut self, ctx: &CtxRef) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.checkbox(&mut self.config.side_enabled.0, "Left panel").changed() {
                    self.resize_requested = true;
                }
                if ui.checkbox(&mut self.config.side_enabled.1, "Right panel").changed() {
                    self.resize_requested = true;
                }
            });
        });
    }
    fn update_buffer(buffer: &mut String, input_state: &Vec<Vec<InputKey>>) {
        for row in input_state {
            for key in row {
                if key.pressed {
                    buffer.push(key.character);
                }
            }
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        self.draw_top_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().window_shadow = self.config.style.window_shadow;
            match &mut self.config.side_enabled {
                (false, false) => {
                    App::draw_about_window(ctx);
                },
                (false, true) => {
                    self.draw_right_panel(ctx);
                },
                (true, false) => {
                    self.draw_left_panel(ctx);
                },
                (true, true) => {
                    self.draw_left_panel(ctx);
                    self.draw_right_panel(ctx);
                },
            }
        });

        if self.resize_requested {
            self.recalculate_size(frame);
            self.resize_requested = false;
        }

        if self.exit_requested {
            frame.quit()
        };
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
                window_shadow: self.config.style.window_shadow,
                ..Default::default()
            },
            animation_time: 0.0,
            debug: Default::default(),
            explanation_tooltips: false
        };

        _ctx.set_style(Arc::new(app_style));
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

