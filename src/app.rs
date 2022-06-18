use eframe::egui::{self, CentralPanel};
use eframe::egui::{ Button, Color32, Context, Stroke, Ui, Vec2,
};
use eframe::Frame;
use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

use crate::components::{TypingPanel, AboutPanel, TopBar, Panel, PanelState, Drawable};
use crate::keyboard::{KeyboardState};
use crate::{ApplicationConfig, StyleConfig};


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub config: ApplicationConfig,
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
        let about_panel = Rc::new(RefCell::new(AboutPanel::default()));

        let mut app = App {
            config: ApplicationConfig::new(),
            top_bar: TopBar {
                info: Panel::new("top bar".to_owned(), PanelState::Enabled),
                about_panel: about_panel.clone(),
                left_panel: left_panel.clone(),
                right_panel: right_panel.clone(),
            },
            left_panel,
            right_panel,
            about_panel,
        };
        app.right_panel.borrow_mut().info.state = PanelState::Disabled;

        app
    }
}

impl App {
    // FIXME: move into an object
    pub(crate) fn draw_keys(style_config: &StyleConfig, ui: &mut Ui, keyboard: &KeyboardState) {
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
                            .stroke(Stroke::new(key.down as i32 as f32, Color32::WHITE)),
                    );
                }
            });
            current_row_indent += style_config.button_indent;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // self.draw_top_bar(ctx, frame);
        self.top_bar.draw(ctx);

        CentralPanel::default().show(ctx, |_ui| {
            if self.about_panel.borrow().info.state == PanelState::Enabled {
                self.about_panel.borrow_mut().draw(ctx);
                return;
            }
            if self.left_panel.borrow().info.state == PanelState::Enabled {
                self.left_panel.borrow_mut().update(ctx);
                self.left_panel.borrow_mut().draw(ctx);
            } else if self.right_panel.borrow().info.state == PanelState::Enabled {
                self.right_panel.borrow_mut().update(ctx);
                self.right_panel.borrow_mut().draw(ctx);
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
        true
    }
}
