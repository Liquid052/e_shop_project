#![ allow(unused)]
use std::any::type_name;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use anymap::AnyMap;
use dotenvy::dotenv;
use log::error;
use simple_logger::SimpleLogger;
use crate::prelude::*;


// aliases
pub type DynPlugin = Arc<RwLock<dyn Plugin>>;
pub type PluginContainer<T> = Arc<RwLock<T>>;

#[derive(Debug)]
pub struct App {
    plugins: HashMap<String, DynPlugin>,
    app_name: String,

    // to ensure order in which the plugins were added
    update_vec: Vec<DynPlugin>,

    any_map: AnyMap,
    res_map: SingletonMap,

}

impl App {
    pub fn new() -> Self {
        // init utils
        SimpleLogger::new().init().unwrap();
        dotenv().ok();

        let mut res_map: SingletonMap = Default::default();
        res_map.insert(AppExit::default());

        App {
            plugins: Default::default(),
            app_name: "".into(),
            any_map: Default::default(),
            res_map,
            update_vec: vec![],
        }
    }

    // plugin management
    pub fn add<T: Plugin>(mut self, mut plugin: T) -> Self {
        let plugin_type = plugin.plugin_name().into();

        plugin.insert_resources(&mut self);

        let arc = Arc::new(RwLock::new(plugin));

        self.plugins.insert(plugin_type, arc.clone());
        self.any_map.insert(arc.clone());
        self.update_vec.push(arc);

        self
    }
    pub fn get<T: Plugin>(&self) -> Option<Arc<RwLock<T>>> {
        self.any_map.get::<Arc<RwLock<T>>>().cloned()
    }
    pub fn get_expected<T: Plugin>(&self) -> Arc<RwLock<T>> {
        let Some(arc) = self.any_map
            .get::<Arc<RwLock<T>>>()
            .cloned() else {

            error!("Attempted to get type that isn't registered in App {}", type_name::<T>());
            panic!();
        };

        arc
    }

    // resource management
    pub fn insert_resource(&mut self, val: impl Debug + Clone + 'static) {
        self.res_map.insert(val);
    }
    pub fn get_resource<T: Clone + Debug + 'static>(&self) -> Option<T> {
        self.res_map.get()
    }
    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.res_map.remove()
    }

    pub fn run(&mut self) {
        self.update_vec.iter()
            .for_each(|plugin| plugin.write().unwrap().on_build(self));

        self.update_vec.iter()
            .for_each(|plugin| plugin.write().unwrap().on_startup());

        // update loop
        while !self.get_resource::<AppExit>()
            .expect("Cant take AppExit")
            .should_exit()
        {
            self.update_vec.iter()
                .for_each(|plugin| plugin.write().unwrap().on_update());

            self.update_vec.iter()
                .for_each(|plugin| plugin.write().unwrap().on_post_update());

        }
    }
}