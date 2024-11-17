use crate::prelude::*;

mod plugins;
mod prelude;
mod resources;

// db
mod models;
mod schema;

fn main() {
    AppBuilder::new()
        .name("Eshop project")
        .add(DatabasePlugin::new())
        .add(DomainPlugin::new())
        .add(BusinessPlugin::new())
        .add(WindowPlugin::new())
        .build()
        .run();
}
