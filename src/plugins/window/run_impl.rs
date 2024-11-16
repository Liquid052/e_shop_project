use crate::prelude::{Run, WindowPlugin};

const SIZE: (f32, f32) = (900.0, 500.0);

impl Run for WindowPlugin {
    fn run(self) {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([SIZE.0, SIZE.1])
                .with_min_inner_size([SIZE.0, SIZE.1])
                .with_transparent(true), // To have rounded corners we need transparency
            ..Default::default()
        };

        let name = self.app.as_ref().unwrap().get_name().to_string();
        eframe::run_native(
            name.as_str(),
            native_options,
            Box::new(|cc| Ok(Box::new(self))),
        )
        .unwrap();
    }
}
