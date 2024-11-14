use std::sync::RwLock;
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct WindowPlugin {
    pub db: Option<PluginContainer<DatabasePlugin>>,
}

impl Plugin for WindowPlugin {
    fn plugin_name(&self) -> &'static str {
        "window"
    }

    fn on_build(&mut self, app: &App) {
        let db = app.get_expected::<DatabasePlugin>();
        self.db = Some(db);

        info!("works");
    }
}