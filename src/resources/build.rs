use crate::prelude::*;

pub enum Build {
    Window(WindowPlugin),
    App(App)
}

impl Build {
    pub fn run(mut self) {
        match self {
            Build::Window(window) => window.run(),
            Build::App(app) => app.run()
        }
    }
}