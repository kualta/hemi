use eframe::{egui, epi};
use eframe::egui::{CentralPanel, Color32, CtxRef, Pos2, Vec2, Window};
use eframe::egui::Event::Key;
use eframe::egui::Key::A;
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
                ui.label("HI");
        });
    }

    fn draw_about_window(ctx: &CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Built with egui");
            ui.label("lectroMathew, 2022");
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
