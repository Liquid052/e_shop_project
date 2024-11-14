use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use plugin::Plugin;

pub mod plugin;

// aliases
pub type PluginContainer = Arc<RwLock<dyn Plugin>>;

#[derive(Default, Debug)]
pub struct App {
    plugins: HashMap<String, PluginContainer>,

    type_map:     HashMap<TypeId, PluginContainer>,
    update_order: Vec<PluginContainer>,
}

impl App {
    pub fn new() -> Self {
        App {
            ..Self::default()
        }
    }

    pub fn add_plugin<T: Plugin>(mut self, plugin: T) -> Self {
        let plugin_type = plugin.plugin_name().into();
        let arc = Arc::new(RwLock::new(plugin));

        let type_id = TypeId::of::<T>();

        self.plugins.insert(plugin_type, arc.clone());
        self.type_map.insert(type_id, arc.clone());
        self.update_order.push(arc);

        self
    }

    pub fn get_plugin<T: Plugin>(&self) -> Option<Arc<RwLock<T>>> {
        self.type_map
            .get(&TypeId::of::<T>())
            .map(|arc| Arc::clone(arc) as Arc<RwLock<T>>)
    }



    pub fn run(&mut self) {
        for plugin_type in &self.update_order {
            if let Some(plugin) = self.plugins.get(plugin_type) {
                plugin.write().unwrap().on_update(self);
            }
        }
    }
}