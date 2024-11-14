use crate::app::App;
use crate::app::app_exit::AppExit;
use crate::app::plugin::Plugin;


mod schema;
mod models;

#[derive(Debug, Default)]
pub struct DatabasePlugin {
    app_exit: AppExit
}

impl Plugin for DatabasePlugin {
    fn plugin_name(&self) -> &'static str {
        "database_layer"
    }

    fn on_build(&mut self, app: &App) {
        self.app_exit = app.get_app_exit();
    }

    fn on_post_update(&mut self) {
        self.app_exit.set(true);
    }

}