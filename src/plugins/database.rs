#![allow(dead_code)]
use crate::prelude::*;
use diesel::{Connection, PgConnection};
use std::fmt::{Debug, Formatter};
use std::env;
use log::info;

pub struct DatabasePlugin {
    app_exit: AppExit,
    conn:     PgConnection,
}

// manual debug impl since PgConnection doesn't support it
impl Debug for DatabasePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database plugin")
            .field("app_exit", &self.app_exit)
            .finish()
    }
}

impl DatabasePlugin {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE URL must be set");

        let conn = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        info!("db initialized");

        Self {
            app_exit: Default::default(),
            conn
        }
    }
}

impl Plugin for DatabasePlugin {
    fn plugin_name(&self) -> &'static str {
        "database_layer"
    }

    fn on_build(&mut self, app: &App) {
        self.app_exit = app.get_resource::<AppExit>().unwrap();

        // let results: Vec<Account> = dsl::accounts
        //     .limit(5)
        //     .load(&mut *self.conn)
        //     .expect("Error loading posts");
        // info!("results: {:#?}", results[0]);
        // diesel::insert_into(accounts::table)
        //     .values(acc)
        //     .returning(Account::as_returning())
        //     .get_result(&mut *self.conn)
        //     .expect("Error saving account");
        // let ret = schema::accounts::dsl::accounts.find(0)
        //     .select(Account::as_select())
        //     .first(&mut *self.conn)
        //     .optional();
        // info!("connecting: {:?}", ret);

    }

    fn on_post_update(&mut self) {
        self.app_exit.set(true);
    }

}