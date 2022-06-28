use eframe::egui::Context;
use eframe::egui::{self, CentralPanel};
use eframe::Frame;
use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use crate::components::{AboutPanel, TopBar, TypingPanel, AppPanels};
use crate::drawable::Drawable;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    active_panel: Rc<RefCell<AppPanels>>,
    left_panel: Rc<RefCell<TypingPanel>>,
    right_panel: Rc<RefCell<TypingPanel>>,
    about_panel: Rc<RefCell<AboutPanel>>,
    top_bar: TopBar,
}

impl Default for App {
    fn default() -> Self {
        const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
        const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";

        let left_panel = Rc::new(RefCell::new(TypingPanel::new(LEFT_QWERTY_KEYS)));
        let right_panel = Rc::new(RefCell::new(TypingPanel::new(RIGHT_QWERTY_KEYS)));
        let active_panel = Rc::new(RefCell::new(AppPanels::LeftTypingPanel));
        let about_panel = Rc::new(RefCell::new(AboutPanel { }));

        let app = App {
            top_bar: TopBar {
                active_panel: active_panel.clone(),
                left_panel: left_panel.clone(),
                right_panel: right_panel.clone(),
            },
            left_panel,
            right_panel,
            about_panel,
            active_panel,
        };

        app
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {

            self.top_bar.draw(frame, ui);

            match *self.active_panel.borrow_mut() {
                AppPanels::LeftTypingPanel => {
                    self.left_panel.borrow_mut().update(ctx);
                    self.left_panel.borrow_mut().draw(frame, ui);
                }
                AppPanels::RightTypingPanel => {
                    self.right_panel.borrow_mut().update(ctx);
                    self.right_panel.borrow_mut().draw(frame, ui);
                }
                AppPanels::AboutPanel => {
                    self.about_panel.borrow_mut().draw(frame, ui);
                }
            }
        });
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        true
    }
}
