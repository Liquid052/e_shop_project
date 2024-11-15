use std::error::Error;
use diesel::pg::Pg;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("/eshop/");


fn run_migrations(connection: &mut PgConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    // connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}