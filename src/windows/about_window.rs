use std::marker::PhantomData;
use eframe::egui;
use eframe::egui::{CtxRef, Ui};
use crate::App;
use crate::drawable::Drawable;


pub struct AboutWindow<'app> {
    _marker: PhantomData<&'app ()>
}

impl<'app> AboutWindow<'app> {
    pub(crate) fn new() -> AboutWindow<'app> {
        AboutWindow {
            _marker: Default::default()
        }
    }
}


impl<'app> Drawable<'app> for AboutWindow<'app> {
    fn draw(&self, ctx: &CtxRef, app: &mut App) {

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

impl<'app> Drawable<'app> for &AboutWindow<'app> {
    fn draw(&self, ctx: &CtxRef, app: &mut App) {

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