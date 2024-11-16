use crate::prelude::*;

pub enum Build {
    App(App),
    Window(WindowPlugin),
}

impl Build {
    pub fn run(mut self) {
        match self {
            Build::App(app) => app.run(),
            Build::Window(window_plugin) => window_plugin.run(),
        }
    }
}
