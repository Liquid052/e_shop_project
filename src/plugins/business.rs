use crate::models::{Account, Order, Product, ProductComment};
use crate::plugins::{DatabasePlugin, DomainPlugin};
use crate::prelude::App;
use crate::resources::Plugin;
use log::{error, info, trace};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct BusinessPlugin {
    domain: Option<Arc<RwLock<DomainPlugin>>>,
}

impl BusinessPlugin {
    // constructors
    pub fn new() -> Self {
        Self { domain: None }
    }

    // api
    pub fn search_products(&mut self, search: String) -> Vec<Product> {
        let domain = self.domain.as_ref().unwrap();
        let search = search.to_lowercase();

        domain
            .write()
            .unwrap()
            .get_all_products()
            .into_iter()
            .filter(|product| product.name.to_lowercase().contains(&search))
            .collect()
    }
    pub fn search_accounts(&mut self, search: String) -> Vec<Account> {
        let domain = self.domain.as_ref().unwrap();
        let search = search.to_lowercase();

        domain
            .write()
            .unwrap()
            .get_all_accounts()
            .into_iter()
            .filter(|account| account.username.to_lowercase().contains(&search))
            .collect()
    }
    pub fn get_all_orders(&mut self) -> Vec<Order> {
        let domain = self.domain.as_ref().unwrap();

        domain.write().unwrap().get_all_orders()
    }
    pub fn get_order_details(&mut self, order: &Order) -> Option<(Account, Vec<(usize, Product)>)> {
        let domain = self.domain.as_ref().unwrap();

        domain.write().unwrap().get_order_details(order)
    }

    pub fn get_comments(&mut self, product: &Product) -> Vec<ProductComment> {
        let domain = self.domain.as_ref().unwrap();

        domain
            .write()
            .unwrap()
            .get_all_comments()
            .into_iter()
            .filter(|comment| comment.product_id == product.product_id)
            .collect()
    }

    pub fn add_comment(&mut self, username: String, product: &Product, text: String) {
        if text.len() < 20 {
            error!("Error adding comment: Minimal length is 20");
            return;
        }
        let domain = self.domain.as_ref().unwrap();
        let mut lock = domain.write().unwrap();
        let Some(account) = lock.get_account_by_username(&username) else {
            error!("Error adding comment: username {} was not found", username);
            return;
        };

        lock.insert_comment(&account, product, text);

        trace!("comment added");
    }
    pub fn cancel_order(&mut self, order: &Order) {
        self.domain
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .delete_order(order);
    }
    pub fn delete_account(&mut self, account: &Account) {
        self.domain
            .as_ref()
            .unwrap()
            .write()
            .unwrap()
            .delete_account(account);
    }
}

impl Plugin for BusinessPlugin {
    fn plugin_name(&self) -> &'static str {
        "domain_plugin"
    }
    fn on_build(&mut self, app: &App) {
        self.domain = Some(app.get().unwrap());

        info!("business initialized");
    }
}
