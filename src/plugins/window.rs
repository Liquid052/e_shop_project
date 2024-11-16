use crate::prelude::*;
use std::default::Default;
use std::error::Error;

mod eframe_impl;
mod plugin_impl;
mod run_impl;
mod states;

#[derive(Debug)]
pub struct WindowPlugin {
    pub(crate) app: Option<App>,
    app_exit: AppExit,
    update: UpdateGui,
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
