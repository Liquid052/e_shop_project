use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::app::App;

mod app;
mod plugins;
mod prelude;

fn main() {
    App::new()
        .add_plugin()
        .run();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set");



    println!("Hello, world!");
}
