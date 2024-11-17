use crate::plugins::WindowPlugin;
use crate::prelude::Plugin;

impl Plugin for WindowPlugin {
    fn plugin_name(&self) -> &'static str {
        "window"
    }
}
