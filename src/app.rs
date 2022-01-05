use eframe::{egui, epi};
use eframe::egui::{CentralPanel, Color32, Pos2, Window};
use crate::drawable::Drawable;
use crate::windows::main_window::*;
use crate::windows::about_window::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App<'app> {
    pub(crate) active_window: Option<&'app dyn Drawable<'app>>,
    pub(crate) main_window: MainWindow<'app>,
    pub(crate) about_window: AboutWindow<'app>,
    pub(crate) exit_requested: bool,
}

impl<'app> Default for App<'app> {
    fn default() -> Self {
        Self {
            active_window: Option::None,
            main_window: MainWindow::new(),
            about_window: AboutWindow::new(),
            exit_requested: false,
        }
    }
}

impl<'app> App<'app> {
    pub(crate) fn set_active_window(&mut self, window: &'app dyn Drawable<'app>) {
        self.active_window = Some(window);
    }
    pub(crate) fn exit(&mut self) {
        self.exit_requested = true;
    }
}

impl<'app> epi::App for App<'app> {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { .. } = self;

        match &mut self.active_window {
            Some(w) => {
                w.draw(ctx, self);
            }
            None => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("No active window is selected!");
                });
            }
        };

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

        // self.main_window = Some(MainWindow::new(&self));
        // self.active_window = Some(Box::new(&self.main_window));
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
