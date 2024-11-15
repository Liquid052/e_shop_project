use crate::prelude::*;

#[derive(Debug)]
pub struct WindowPlugin {
    pub db: Option<PluginContainer<DatabasePlugin>>,
    pub update: UpdateGui,
}

impl WindowPlugin {
    pub fn new() -> Self {
        Self {
            db:             None,
            update: Default::default(),
        }
    }
}

impl Plugin for WindowPlugin {
    fn plugin_name(&self) -> &'static str {
        "window"
    }

    fn insert_resources(&mut self, app: &mut App) {
        let update_gui = UpdateGui::default();

        self.update = update_gui.clone();
        app.insert_resource(update_gui);
    }

    fn on_build(&mut self, app: &App) {
        let db = app.get_expected::<DatabasePlugin>();
        self.db = Some(db);

        info!("window initialized");
    }
    fn on_update(&mut self) {
        if self.update.should_update() {

        }
    }
}


