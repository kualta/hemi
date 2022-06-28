use crate::components::{AboutPanel, AppPanels, KeyboardPanel, PanelState, TopBar, TypingPanel};
use eframe::egui::{self, Button, CentralPanel, Ui};
use eframe::egui::{Align, Align2, RichText, Vec2};
use eframe::epaint::Stroke;
use eframe::Frame;
use std::default::Default;
pub trait Drawable {
    fn draw(&mut self, frame: &mut Frame, ui: &mut Ui);
}

impl Drawable for AboutPanel {
    fn draw(&mut self, frame: &mut Frame, ui: &mut Ui) {
        CentralPanel::default().show(ui.ctx(), |ui| {
            egui::Area::new("about_area")
                .anchor(Align2::CENTER_CENTER, Vec2::new(0.0, 0.0))
                .show(ui.ctx(), |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.horizontal(|ui| {
                        egui::Layout::top_down_justified(Align::Center);
                        ui.label("made by ");
                        ui.hyperlink_to("lectro.moe", "https://lectro.moe/");
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
}

impl Drawable for TypingPanel {
    fn draw(&mut self, frame: &mut Frame, ui: &mut Ui) {
        egui::Window::new(&self.info.title)
            .resizable(false)
            .title_bar(false)
            .collapsible(false)
            .auto_sized()
            .show(ui.ctx(), |ui| {
                egui::TopBottomPanel::top(&self.info.title)
                    .resizable(false)
                    .height_range(250. ..=250.)
                    .show_inside(ui, |ui| {
                        ui.add_space(125.);
                        ui.horizontal(|ui| {
                            ui.add_space(75.);
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text.last_word().unwrap_or(&"".to_owned()),
                                )),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::widgets::TextEdit::singleline(&mut self.text.input_buffer)
                                    .cursor_at_end(true),
                            );
                            ui.add_sized(
                                Vec2::new(100., 30.),
                                egui::Label::new(RichText::from(
                                    self.text.next_word().unwrap_or(&"".to_owned()),
                                )),
                            );
                        });
                    });
                ui.add_space(120.);
                self.keyboard_panel.draw(frame, ui);
            });
    }
}

impl Drawable for TopBar {
    fn draw(&mut self, frame: &mut Frame, ui: &mut Ui) {
        let mut active_panel = self.active_panel.borrow_mut();

        egui::TopBottomPanel::top("top_bar").show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Switch side").clicked() {
                    *active_panel = match *active_panel {
                        AppPanels::LeftTypingPanel => AppPanels::RightTypingPanel,
                        AppPanels::RightTypingPanel => AppPanels::LeftTypingPanel,
                        AppPanels::AboutPanel => AppPanels::LeftTypingPanel,
                    };
                }
                if ui.button("Keyboard").clicked() {
                    self.left_panel.borrow_mut().keyboard_panel.state.reverse();
                    self.right_panel.borrow_mut().keyboard_panel.state.reverse();

                    if self.left_panel.borrow().keyboard_panel.state == PanelState::Disabled {
                        frame.set_window_size(Vec2::new(500., 413.));
                    } else {
                        frame.set_window_size(Vec2::new(500., 743.));
                    }
                }
                let about_button_size = Vec2::new(50., 10.);
                ui.allocate_space(ui.available_size() - about_button_size);
                if ui.button("About").clicked() {
                    *active_panel = AppPanels::AboutPanel;
                }
            });
        });
    }
}

impl Drawable for KeyboardPanel {
    fn draw(&mut self, frame: &mut Frame, ui: &mut Ui) {
        if self.state == PanelState::Disabled {
            return;
        }
        ui.spacing_mut().item_spacing = self.button_spacing;
        let mut current_row_indent = 0.;

        for row in &self.keyboard.borrow().rows {
            ui.horizontal(|ui| {
                ui.add_space(current_row_indent);
                for key in row {
                    let width_mul = if key.key == egui::Key::Space { 4.6 } else { 1. };
                    ui.add_sized(
                        Vec2::new(self.button_size * width_mul, self.button_size),
                        Button::new(key.character.to_string())
                            //                   converting bool to either 0. or 1.
                            .stroke(Stroke::new(key.down as i32 as f32, self.stroke_color)),
                    );
                }
            });
            current_row_indent += self.row_indent;
        }
    }
}
