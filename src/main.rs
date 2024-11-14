use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use log::error;
use crate::prelude::*;

mod app;
mod plugins;
mod prelude;

fn main() {
    App::new()
        .add(DatabasePlugin::default())
        .add(WindowPlugin::default())
        .run();

    dotenv().ok();


    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    info!("Hello, world!");
}
