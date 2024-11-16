use crate::prelude::*;

mod resources;
mod plugins;
mod prelude;

// db
mod schema;
mod models;

fn main() {
    AppBuilder::new()
        .name("Eshop project")
        .add(DatabasePlugin::new())
        .add(DomainPlugin::new())
        .add(WindowPlugin::new())
        .build()
        .run();
}
