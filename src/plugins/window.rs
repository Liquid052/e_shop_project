use std::default::Default;
use std::error::Error;
use egui::Context;
use crate::prelude::*;

mod states;
mod eframe_impl;
mod plugin_impl;
mod run_impl;

// export of impls and others
pub use run_impl::*;

#[derive(Debug)]
pub struct WindowPlugin {
    pub(crate) app: Option<App>,
    app_exit: AppExit,
    update:   UpdateGui,
}

impl WindowPlugin {
    pub fn new() -> Self {
        Self {
            app: None,
            app_exit: Default::default(),
            update: Default::default(),
        }
    }
}