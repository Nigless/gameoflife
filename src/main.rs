#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;
use eframe;
mod app;
mod canvas;
mod lib;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2((1920) as f32, (1080) as f32)),
        ..Default::default()
    };
    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(App::new())))
}
