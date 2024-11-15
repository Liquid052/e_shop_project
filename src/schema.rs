// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        username -> Text,
        role -> Text,
        first_name -> Text,
        last_name -> Text,
        phone_number -> Varchar,
        email -> Text,
        credit_card -> Text,
        address_id -> Int4,
    }
}

diesel::table! {
    addresses (address_id) {
        address_id -> Int4,
        street -> Text,
        city -> Text,
        state -> Text,
        postal_code -> Varchar,
        country -> Text,
    }
}

diesel::table! {
    categories (category_id) {
        category_id -> Int4,
        type_name -> Text,
    }
}

diesel::table! {
    ordered_products (ordered_product_id) {
        ordered_product_id -> Int4,
        amount -> Int4,
        order_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        total_price -> Numeric,
        finished -> Bool,
        account_id -> Int4,
    }
}

diesel::table! {
    product_comments (comment_id) {
        comment_id -> Int4,
        content -> Text,
        account_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Int4,
        name -> Text,
        description -> Text,
        price -> Numeric,
        category_id -> Int4,
    }
}

diesel::table! {
    stocks (stock_id) {
        stock_id -> Int4,
        amount -> Int4,
        warehouse_id -> Int4,
        product_id -> Int4,
    }
}

diesel::table! {
    warehouses (warehouse_id) {
        warehouse_id -> Int4,
        name -> Text,
        capacity -> Int4,
        address_id -> Int4,
    }
}

diesel::joinable!(accounts -> addresses (address_id));
diesel::joinable!(ordered_products -> orders (order_id));
diesel::joinable!(ordered_products -> products (product_id));
diesel::joinable!(orders -> accounts (account_id));
diesel::joinable!(product_comments -> accounts (account_id));
diesel::joinable!(product_comments -> products (product_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(stocks -> products (product_id));
diesel::joinable!(stocks -> warehouses (warehouse_id));
diesel::joinable!(warehouses -> addresses (address_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    addresses,
    categories,
    ordered_products,
    orders,
    product_comments,
    products,
    stocks,
    warehouses,
);
