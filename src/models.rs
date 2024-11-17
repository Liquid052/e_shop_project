use crate::schema::*;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::addresses)]
pub struct Address {
    pub address_id: i32,
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Queryable, Selectable, Insertable, Default, Clone, Debug)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub account_id: i32,
    pub username: String,
    pub role: String, // Use a custom enum for this
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub credit_card: String,
    pub address_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = orders)]
pub struct Order {
    pub order_id: i32,
    pub total_price: BigDecimal,
    pub finished: bool,
    pub account_id: i32,
}

#[derive(Queryable, Insertable, Debug, Default, Clone)]
#[diesel(table_name = product_comments)]
pub struct ProductComment {
    pub comment_id: i32,
    pub content: String,
    pub account_id: i32,
    pub product_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::ordered_products)]
pub struct OrderedProduct {
    pub ordered_product_id: i32,
    pub amount: i32,
    pub order_id: i32,
    pub product_id: i32,
}

#[derive(Queryable, Insertable, Default, Clone, Debug)]
#[diesel(table_name = products)]
pub struct Product {
    pub product_id: i32,
    pub name: String,
    pub description: String,
    pub price: BigDecimal,
    pub category_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub category_id: i32,
    pub type_name: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = warehouses)]
pub struct Warehouse {
    pub warehouse_id: i32,
    pub name: String,
    pub capacity: i32,
    pub address_id: i32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = stocks)]
pub struct Stock {
    pub stock_id: i32,
    pub amount: i32,
    pub warehouse_id: i32,
    pub product_id: i32,
}
