#![allow(dead_code)]
use crate::prelude::*;
use diesel::{Connection, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use std::fmt::{Debug, Formatter};
use std::env;
use log::info;
use crate::models::Account;
use crate::plugins::database::migrations::run_migrations;
use crate::schema;

mod migrations;

pub struct DatabasePlugin {
    conn: SqliteConnection,
}

// manual debug impl since PgConnection doesn't support it
impl Debug for DatabasePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database plugin")
            .finish()
    }
}

impl DatabasePlugin {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE URL must be set");

        let mut conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        run_migrations(&mut conn).unwrap();

        info!("db initialized");

        Self {
            conn
        }
    }
}

impl Plugin for DatabasePlugin {
    fn plugin_name(&self) -> &'static str {
        "database_layer"
    }

    fn on_startup(&mut self) {
        use crate::schema::accounts::dsl::*;
        let ret: Vec<Account> = accounts.load(&mut self.conn).unwrap();

        info!("connecting: {:?}", ret);
    }
}