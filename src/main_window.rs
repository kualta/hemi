use eframe::egui;
use eframe::egui::CtxRef;
use crate::drawable::Drawable;

pub struct MainWindow {

}

impl Default for MainWindow {
    fn default() -> Self {
        MainWindow { }
    }
}

impl Drawable for MainWindow {
    fn draw(&self, ctx: &CtxRef) {

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HemiTyper");
        });

    }
}
