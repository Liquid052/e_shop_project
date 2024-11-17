use crate::models::{Account, Order, OrderedProduct, Product, ProductComment};
use crate::plugins::DatabasePlugin;
use crate::prelude::App;
use crate::resources::Plugin;
use log::info;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct DomainPlugin {
    database: Option<Arc<RwLock<DatabasePlugin>>>,
}

impl DomainPlugin {
    // constructors
    pub fn new() -> Self {
        Self { database: None }
    }

    pub fn get_all_products(&mut self) -> Vec<Product> {
        let db = self.database.as_ref().unwrap();

        db.write().unwrap().get_all_products()
    }
    pub fn get_all_orders(&mut self) -> Vec<Order> {
        let db = self.database.as_ref().unwrap();

        db.write().unwrap().get_all_orders()
    }
    pub fn get_all_comments(&mut self) -> Vec<ProductComment> {
        let db = self.database.as_ref().unwrap();

        db.write().unwrap().get_all_comments()
    }
    pub fn get_account_by_username(&mut self, name: &str) -> Option<Account> {
        let db = self.database.as_ref().unwrap();
        let mut account = None;
        db.write()
            .unwrap()
            .get_all_accounts()
            .iter()
            .for_each(|q_account| {
                if q_account.username.as_str().eq(name) {
                    account = Some(q_account.clone());
                }
            });

        account
    }
    pub fn get_account_by_id(&mut self, id: i32) -> Option<Account> {
        let db = self.database.as_ref().unwrap();
        let mut account = None;
        db.write()
            .unwrap()
            .get_all_accounts()
            .iter()
            .for_each(|q_account| {
                if q_account.account_id.eq(&id) {
                    account = Some(q_account.clone());
                }
            });

        account
    }
    pub fn get_order_details(&mut self, order: &Order) -> Option<(Account, Vec<(usize, Product)>)> {
        let db = self.database.as_ref().unwrap();

        let orders: Vec<_> = db
            .write()
            .unwrap()
            .get_all_ordered_products()
            .into_iter()
            .filter(|ordered_product| ordered_product.order_id.eq(&order.order_id))
            .collect();

        let orders: Vec<(usize, Product)> = orders
            .into_iter()
            .map(|ordered_product: OrderedProduct| {
                let product = self.get_product(ordered_product.product_id).unwrap();

                (ordered_product.amount as usize, product)
            })
            .collect();

        let account = self.get_account_by_id(order.account_id).unwrap();

        Some((account, orders))
    }
    fn get_product(&mut self, product_id: i32) -> Option<Product> {
        let db = self.database.as_ref().unwrap();
        let mut product = None;
        db.write()
            .unwrap()
            .get_all_products()
            .iter()
            .for_each(|q_account| {
                if q_account.product_id.eq(&product_id) {
                    product = Some(q_account.clone());
                }
            });

        product
    }

    pub fn get_all_accounts(&mut self) -> Vec<Account> {
        let db = self.database.as_ref().unwrap();

        db.write().unwrap().get_all_accounts()
    }
    pub fn insert_comment(&mut self, acc: &Account, product: &Product, content: String) {
        let db = self.database.as_ref().unwrap();
        let mut lock = db.write().unwrap();
        let account_id = lock.get_all_comments().len() + 1;

        let product_comment = ProductComment {
            comment_id: account_id as i32,
            content,
            account_id: product.product_id,
            product_id: acc.account_id,
        };

        lock.add_comment(product_comment);
    }
    pub fn delete_order(&mut self, order: &Order) {
        self.database
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .delete_order(order);
    }
    pub fn delete_account(&mut self, account: &Account) {
        self.database
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .delete_account(account);
    }
}

impl Plugin for DomainPlugin {
    fn plugin_name(&self) -> &'static str {
        "domain_plugin"
    }
    fn on_build(&mut self, app: &App) {
        self.database = Some(app.get().unwrap());

        info!("domain initialized");
    }
}
