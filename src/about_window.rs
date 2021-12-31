use eframe::egui;
use eframe::egui::{CtxRef, Ui};
use crate::drawable::Drawable;

#[derive(Copy, Clone)]
pub struct AboutWindow {

}

impl Default for AboutWindow {
    fn default() -> Self {
       Self { }
    }
}

impl Drawable for AboutWindow {
    fn draw(&self, ctx: &CtxRef) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                    egui::warn_if_debug_build(ui);
                });
            });
        });

    }
}

impl<'a> Drawable for &'a AboutWindow {
    fn draw(&self, ctx: &CtxRef) {

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }
}