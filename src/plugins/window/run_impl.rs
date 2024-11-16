use crate::prelude::{Run, WindowPlugin};

impl Run for WindowPlugin {
    fn run(self) {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "My egui App",
            native_options,
            Box::new(|cc| Ok(Box::new(self))
            ))
            .unwrap();
    }
}