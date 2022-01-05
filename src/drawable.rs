use eframe::egui::{CtxRef, Ui};
use crate::App;

pub trait Drawable<'app> {
    fn draw(&self, ctx: &CtxRef, app: &mut App);
}