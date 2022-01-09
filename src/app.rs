use eframe::{egui, epi};
use eframe::egui::{CentralPanel, Color32, Pos2, Vec2, Window};

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
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: ApplicationConfig::new(),
            exit_requested: false,
        }
    }
}

impl App {
    pub(crate) fn exit(&mut self) {
        self.exit_requested = true;
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        match &mut self.config.side_enabled {
            (false, false) => {},
            (false, true) => {},
            (true, false) => {},
            (true, true) => {},
        }

        egui::SidePanel::right("my_side_panel")
            .resizable(false)
            .min_width(500.0)
            .show(ctx, |ui| {
                ui.label("Right Panel!");
                ui.label("Right Panel!");
                ui.label("Right Panel!");
                ui.label("Right Panel!");
                ui.label("Right Panel!");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("HI");
        });


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
