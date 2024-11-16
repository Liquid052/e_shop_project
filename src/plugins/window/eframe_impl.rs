use crate::prelude::WindowPlugin;
use eframe::Frame;
use egui::Context;

impl eframe::App for WindowPlugin {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
