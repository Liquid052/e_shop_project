use crate::plugins::{BusinessPlugin, DomainPlugin};
use crate::prelude::{Plugin, Run, WindowPlugin};
use log::info;

const SIZE: (f32, f32) = (900.0, 500.0);
const MIN_SIZE: (f32, f32) = (400.0, 300.0);

impl Run for WindowPlugin {
    fn run(mut self) {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([SIZE.0, SIZE.1])
                .with_min_inner_size([MIN_SIZE.0, MIN_SIZE.1])
                .with_resizable(false), // To have rounded corners we need transparency
            ..Default::default()
        };

        self.app.as_mut().unwrap().build();
        self.business = Some(self.app.as_ref().unwrap().get::<BusinessPlugin>().unwrap());
        info!("window initialized");

        let name = self.app.as_ref().unwrap().get_name().to_string();
        eframe::run_native(
            name.as_str(),
            native_options,
            Box::new(|_cc| Ok(Box::new(self))),
        )
        .unwrap();
    }
}
