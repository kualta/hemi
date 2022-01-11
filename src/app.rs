use std::alloc::Layout;
use std::collections::hash_map::IntoKeys;
use eframe::{egui, epi};
use eframe::egui::{Align, Align2, Button, CentralPanel, Color32, CtxRef, Pos2, Stroke, Vec2, Window};
use eframe::egui::Event::Key;
use eframe::egui::WidgetType::ColorButton;
use eframe::epi::Frame;

pub struct ApplicationConfig {
    pub side_enabled: (bool, bool),
}

impl ApplicationConfig {
    pub fn new() -> Self {
        ApplicationConfig {
            side_enabled: (true, true),
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: ApplicationConfig::new(),
            exit_requested: false,
            resize_requested: false,
        }
    }
}

fn get_key_from_str(key: &str) -> egui::Key {
    match key {
        "A" => egui::Key::A,
        "B" => egui::Key::B,
        "C" => egui::Key::C,
        "D" => egui::Key::D,
        "E" => egui::Key::E,
        "F" => egui::Key::F,
        "G" => egui::Key::G,
        "H" => egui::Key::H,
        "I" => egui::Key::I,
        "J" => egui::Key::J,
        "K" => egui::Key::K,
        "L" => egui::Key::L,
        "M" => egui::Key::M,
        "N" => egui::Key::N,
        "O" => egui::Key::O,
        "P" => egui::Key::P,
        "Q" => egui::Key::Q,
        "R" => egui::Key::R,
        "S" => egui::Key::S,
        "T" => egui::Key::T,
        "U" => egui::Key::U,
        "V" => egui::Key::V,
        "W" => egui::Key::W,
        "X" => egui::Key::X,
        "Y" => egui::Key::Y,
        "Z" => egui::Key::Z,
        " " => egui::Key::Space,
        _ => egui::Key::Space
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
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .min_width(500.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                // egui::Area::new("right_words_window")
                //     .fixed_pos(Pos2::new(10., 10.))
                //     .show(ctx, |ui| {
                //         ui.label("Right Panel!");
                //         ui.label("Right Panel!");
                //         ui.label("Right Panel!");
                //         ui.label("Right Panel!");
                //         ui.label("Right Panel!");
                //     })
            });
    }

    fn draw_left_panel(&mut self, ctx: &CtxRef) {
        // using CentralPanel because SidePanel::left adds additional width and doesn't end in the center
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("text_panel")
                    .resizable(false)
                    .default_height(300.)
                    .show(ctx, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label("HIIHIHIHIHIkjll");
                        })
                    });
                ui.allocate_space(Vec2::new(0., 400.));

                let pressed = ui.input().key_down(egui::Key::A);
                ui.add_sized(Vec2::new(50., 50.), Button::new("A")
                    .stroke(Stroke::new(pressed as i32 as f32, Color32::WHITE))
                );
        });
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
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        self.draw_top_bar(ctx);

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
                self.draw_right_panel(ctx);
                self.draw_left_panel(ctx);
            },
        }

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
