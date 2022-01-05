use std::marker::PhantomData;
use eframe::egui;
use eframe::egui::{CtxRef, Pos2};
use eframe::egui::panel::Side;
use crate::App;
use crate::drawable::Drawable;

pub struct MainWindow<'app> {
    _marker: PhantomData<&'app ()>
}

impl<'app> MainWindow<'_> {
    pub(crate) fn new() -> MainWindow<'app> {
        MainWindow {
            _marker: Default::default()
        }
    }
}

impl Drawable<'_> for MainWindow<'_> {
    fn draw(&self, ctx: &CtxRef, app: &mut App) {

        egui::SidePanel::new(Side::Left, "left_panel").show(ctx, |ui| {
            ui.label("HemiTyper").rect.set_center(Pos2::new(100., 100.));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("HemiTyper").rect.set_center(Pos2::new(100., 100.));
        });

        egui::SidePanel::new(Side::Right, "right_panel").show(ctx, |ui| {
            ui.label("HemiTyper").rect.set_center(Pos2::new(100., 100.));
        });

    }
}
