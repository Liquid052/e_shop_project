use crate::plugins::WindowPlugin;
use crate::prelude::{App, AppExit, Plugin, UpdateGui};

impl Plugin for WindowPlugin {
    fn plugin_name(&self) -> &'static str {
        "window"
    }

    fn insert_resources(&mut self, app: &mut App) {
        self.app_exit = app.get_resource::<AppExit>().unwrap();

        let update_gui = UpdateGui::default();
        self.update = update_gui.clone();
        app.insert_resource(update_gui);
    }
}
