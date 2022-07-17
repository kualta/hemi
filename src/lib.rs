#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
// #![allow(unused, dead_code)]
#![allow(unused_variables)]

mod app;
mod components;
mod drawable;
mod keyboard;

use app::App;
use eframe::egui::Vec2;
use eframe::epaint::Shadow;
use eframe::{
    egui::{Style, Visuals},
};
use std::sync::Arc;

pub fn main() -> ! {
    let app = App::default();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(500.0, 743.0)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "Hemi Typer",
        native_options,
        Box::new(|cc| {
            #[cfg(feature = "persistence")]
            if let Some(storage) = _storage {
                *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
            }

            let app_style = Style {
                visuals: Visuals {
                    window_shadow: Shadow {
                        extrusion: 0.,
                        color: Default::default(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            };
            cc.egui_ctx.set_style(Arc::new(app_style));

            Box::new(app)
        }),
    );
}