use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use diesel::{RunQueryDsl, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, FileBasedMigrations, MigrationHarness};
use diesel::connection::SimpleConnection;
use crate::prelude::*;
use log::info;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(connection: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let vec = connection.run_pending_migrations(MIGRATIONS)?;

    info!("{:?}", vec);

    Ok(())
}
