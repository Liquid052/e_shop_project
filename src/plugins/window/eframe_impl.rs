use crate::plugins::window::states::show_state;
use crate::prelude::*;
use eframe::Frame;
use egui::Context;

impl eframe::App for WindowPlugin {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let state = &mut self.state;
        let app = self.app.as_mut().unwrap();

        app.update();

        {
            let domain_arc = self.business.as_ref().unwrap();
            let mut lock = domain_arc.write().unwrap();
            let business = &mut *lock;

            egui::CentralPanel::default().show(ctx, |ui| {
                show_state(state, business, &mut self.cache, ui);
            });
        }
    }
}
