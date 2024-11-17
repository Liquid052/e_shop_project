#![allow(dead_code)]
use crate::models::{Account, Address, Order, ProductComment};
use crate::plugins::database::migrations::run_migrations;
use crate::prelude::*;
use crate::schema::ordered_products::dsl::ordered_products;
use crate::{models, schema};
use diesel::dsl::insert_into;
use diesel::{
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
    SqliteConnection,
};
use log::info;
use std::env;
use std::fmt::{Debug, Formatter};

mod migrations;

pub struct DatabasePlugin {
    conn: SqliteConnection,
}

// manual debug impl since PgConnection doesn't support it
impl Debug for DatabasePlugin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database plugin").finish()
    }
}

impl DatabasePlugin {
    // Constructors
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

        let mut conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        run_migrations(&mut conn).unwrap();

        info!("db initialized");

        Self { conn }
    }

    pub fn get_all_products(&mut self) -> Vec<models::Product> {
        use crate::schema::products::dsl::*;

        products.load(&mut self.conn).unwrap()
    }
    pub fn get_all_accounts(&mut self) -> Vec<models::Account> {
        use crate::schema::accounts::dsl::*;

        accounts.load(&mut self.conn).unwrap()
    }
    pub fn get_all_comments(&mut self) -> Vec<models::ProductComment> {
        use crate::schema::product_comments::dsl::*;

        product_comments.load(&mut self.conn).unwrap()
    }
    pub fn get_all_addresses(&mut self) -> Vec<models::Address> {
        use crate::schema::addresses::dsl::*;

        addresses.load(&mut self.conn).unwrap()
    }
    pub fn get_all_ordered_products(&mut self) -> Vec<models::OrderedProduct> {
        use crate::schema::ordered_products::dsl::*;

        ordered_products.load(&mut self.conn).unwrap()
    }
    pub fn get_all_orders(&mut self) -> Vec<models::Order> {
        use crate::schema::orders::dsl::*;

        orders.load(&mut self.conn).unwrap()
    }

    pub fn add_comment(&mut self, comment: ProductComment) {
        use crate::schema::product_comments::dsl::*;

        insert_into(product_comments)
            .values(comment)
            .execute(&mut self.conn)
            .unwrap();
    }

    pub fn delete_order(&mut self, order: &Order) {
        use crate::schema::orders::dsl::*;

        diesel::delete(orders.filter(order_id.eq(order.order_id)))
            .execute(&mut self.conn)
            .unwrap();
    }
    pub fn delete_account(&mut self, account: &Account) {
        use crate::schema::accounts::dsl::*;

        diesel::delete(accounts.filter(account_id.eq(account.account_id)))
            .execute(&mut self.conn)
            .unwrap();
    }
}

impl Plugin for DatabasePlugin {
    fn plugin_name(&self) -> &'static str {
        "database_layer"
    }

    fn on_startup(&mut self) {
        // use crate::schema::accounts::dsl::*;
        // let ret: Vec<Account> = accounts.load(&mut self.conn).unwrap();

        // info!("connecting: {:?}", ret);
    }
}
