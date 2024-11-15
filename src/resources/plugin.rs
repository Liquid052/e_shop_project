use std::any::Any;
use std::fmt::Debug;
use crate::resources::App;

pub trait Plugin: Any + Debug + 'static {
    fn plugin_name(&self) -> &'static str;
    fn insert_resources(&mut self, _app: &mut App) {}

    // default implementations as some plugins may not need to implement all of them
    fn on_build(&mut self, _app: &App) {}
    fn on_startup(&mut self) {}
    fn on_update(&mut self) {}
    fn on_post_update(&mut self) {}
}