use crate::prelude::*;
use crate::resources::build::Build;
use anymap::AnyMap;
use std::any::TypeId;

pub struct AppBuilder {
    app: App,
    any_map: AnyMap,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            app: App::new(),
            any_map: Default::default(),
        }
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.app = self.app.name(name);

        self
    }
    pub fn add<T: Plugin>(mut self, mut plugin: T) -> Self {
        if TypeId::of::<T>() == TypeId::of::<WindowPlugin>() {
            plugin.insert_resources(&mut self.app);
            self.any_map.insert(plugin);
        } else {
            self.app = self.app.add(plugin);
        }

        self
    }

    pub fn build(mut self) -> Build {
        let Some(mut window_plugin) = self.any_map.remove::<WindowPlugin>() else {
            return Build::App(self.app);
        };

        window_plugin.app = Some(self.app);
        Build::Window(window_plugin)
    }
}
