use crate::plugins::window::states::WindowState;
use crate::prelude::*;
use anymap::AnyMap;
use std::default::Default;
use std::error::Error;
use std::sync::{Arc, RwLock};

mod eframe_impl;
mod plugin_impl;
mod run_impl;
mod states;

#[derive(Debug)]
pub struct WindowPlugin {
    pub(crate) app: Option<App>,
    pub(crate) cache: AnyMap,
    business: Option<Arc<RwLock<BusinessPlugin>>>,
    state: WindowState,
}

impl WindowPlugin {
    pub fn new() -> Self {
        Self {
            app: None,
            business: None,
            cache: Default::default(),
            state: Default::default(),
        }
    }
}
