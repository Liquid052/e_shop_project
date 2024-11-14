use std::any::TypeId;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use anymap::AnyMap;
use log::error;
use simple_logger::SimpleLogger;
use plugin::Plugin;
use crate::app::app_exit::AppExit;

pub mod plugin;
pub mod app_exit;

// aliases
pub type DynPlugin = Arc<RwLock<dyn Plugin>>;
pub type PluginContainer<T> = Arc<RwLock<T>>;

#[derive(Default, Debug)]
pub struct App {
    plugins: HashMap<String, DynPlugin>,

    type_map:     AnyMap,
    // to ensure order in which the plugins were added
    update_vec: Vec<DynPlugin>,

    app_exit: AppExit
}

impl App {
    pub fn new() -> Self {
        SimpleLogger::new().init().unwrap();

        App {
            ..Self::default()
        }
    }

    pub fn add<T: Plugin>(mut self, plugin: T) -> Self {
        let plugin_type = plugin.plugin_name().into();
        let arc = Arc::new(RwLock::new(plugin));

        let type_id = TypeId::of::<T>();

        self.plugins.insert(plugin_type, arc.clone());
        self.type_map.insert(arc.clone());
        self.update_vec.push(arc);

        self
    }

    // getters
    pub fn get<T: Plugin>(&self) -> Option<Arc<RwLock<T>>> {
        self.type_map.get::<Arc<RwLock<T>>>().cloned()
    }
    pub fn get_expected<T: Plugin>(&self) -> Arc<RwLock<T>> {
        let Some(arc) = self.type_map
            .get::<Arc<RwLock<T>>>()
            .cloned() else {


            panic!("");
        };

        return arc;
    }
    pub fn get_app_exit(&self) -> AppExit {
        self.app_exit.clone()
    }


    pub fn run(&mut self) {


        self.update_vec.iter()
            .for_each(|plugin| plugin.write().unwrap().on_build(self));

        self.update_vec.iter()
            .for_each(|plugin| plugin.write().unwrap().on_startup());

        // update loop
        while !self.app_exit.should_exit() {
            self.update_vec.iter()
                .for_each(|plugin| plugin.write().unwrap().on_update());

            self.update_vec.iter()
                .for_each(|plugin| plugin.write().unwrap().on_post_update());

        }
    }
}