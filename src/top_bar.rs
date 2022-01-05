use std::borrow::Borrow;
use std::ops::Deref;
use eframe::egui;
use eframe::egui::CtxRef;
use crate::{App, app};
use crate::drawable::Drawable;

struct TopBar<'a> {
    app: Box<App<'a>>,
}

impl<'a> TopBar<'a> {
    fn new(app: Box<App>) -> TopBar {
        TopBar {
            app,
        }
    }
}

impl Drawable<'_> for TopBar<'_> {
    fn draw(&self, ctx: &CtxRef, app: &mut App) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        app.exit();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        // app.set_active_window(Box::new(app.about_window.unwrap()));
                    }
                });
            });
        });

    }
}