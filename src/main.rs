#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::egui::Vec2;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = HemiTyper::App::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(1000.0, 800.0));

    eframe::run_native(Box::new(app), native_options);
}
