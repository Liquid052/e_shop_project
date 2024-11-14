use std::any::Any;
use std::fmt::Debug;
use crate::app::App;

pub trait Plugin: Any + Debug + 'static {
    fn plugin_name(&self) -> &'static str;

    fn on_build(&mut self, app: &App) {

    }

    fn on_startup(&mut self) {

    }

    fn on_update(&mut self) {

    }

    fn on_post_update(&mut self) {

    }
}