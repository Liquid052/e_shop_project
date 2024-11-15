use crate::prelude::*;

mod resources;
mod plugins;
mod prelude;

// db
mod schema;
mod models;

fn main() {
    App::new()
        .add(DatabasePlugin::new())
        .add(WindowPlugin::new())
        .run();
}
