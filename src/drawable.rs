use eframe::egui::{CtxRef, Ui};

pub trait Drawable {
    fn draw(&self, ctx: &CtxRef);
}