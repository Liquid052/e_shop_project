-- First create all tables without foreign key constraints
CREATE TABLE "ordered_products" (
                                    "ordered_product_id" INT4 NOT NULL PRIMARY KEY,
                                    "amount" INT4 NOT NULL,
                                    "order_id" INT4 NOT NULL,
                                    "product_id" INT4 NOT NULL
);

CREATE TABLE "product_comments" (
                                    "comment_id" INT4 NOT NULL PRIMARY KEY,
                                    "content" TEXT NOT NULL,
                                    "account_id" INT4 NOT NULL,
                                    "product_id" INT4 NOT NULL
);

CREATE TABLE "stocks" (
                          "stock_id" INT4 NOT NULL PRIMARY KEY,
                          "amount" INT4 NOT NULL,
                          "warehouse_id" INT4 NOT NULL,
                          "product_id" INT4 NOT NULL
);

CREATE TABLE "accounts" (
                            "account_id" INT4 NOT NULL PRIMARY KEY,
                            "username" TEXT NOT NULL,
                            "role" TEXT NOT NULL,
                            "first_name" TEXT NOT NULL,
                            "last_name" TEXT NOT NULL,
                            "phone_number" VARCHAR NOT NULL,
                            "email" TEXT NOT NULL,
                            "credit_card" TEXT NOT NULL,
                            "address_id" INT4 NOT NULL
);

CREATE TABLE "products" (
                            "product_id" INT4 NOT NULL PRIMARY KEY,
                            "name" TEXT NOT NULL,
                            "description" TEXT NOT NULL,
                            "price" NUMERIC NOT NULL,
                            "category_id" INT4 NOT NULL
);

CREATE TABLE "addresses" (
                             "address_id" INT4 NOT NULL PRIMARY KEY,
                             "street" TEXT NOT NULL,
                             "city" TEXT NOT NULL,
                             "state" TEXT NOT NULL,
                             "postal_code" VARCHAR NOT NULL,
                             "country" TEXT NOT NULL
);

CREATE TABLE "warehouses" (
                              "warehouse_id" INT4 NOT NULL PRIMARY KEY,
                              "name" TEXT NOT NULL,
                              "capacity" INT4 NOT NULL,
                              "address_id" INT4 NOT NULL
);

CREATE TABLE "orders" (
                          "order_id" INT4 NOT NULL PRIMARY KEY,
                          "total_price" NUMERIC NOT NULL,
                          "finished" BOOL NOT NULL,
                          "account_id" INT4 NOT NULL
);

CREATE TABLE "categories" (
                              "category_id" INT4 NOT NULL PRIMARY KEY,
                              "type_name" TEXT NOT NULL
);

-- Then add all foreign key constraints
ALTER TABLE "ordered_products"
    ADD CONSTRAINT "fk_ordered_products_order"
        FOREIGN KEY ("order_id") REFERENCES "orders"("order_id"),
    ADD CONSTRAINT "fk_ordered_products_product"
    FOREIGN KEY ("product_id") REFERENCES "products"("product_id");

ALTER TABLE "product_comments"
    ADD CONSTRAINT "fk_product_comments_account"
        FOREIGN KEY ("account_id") REFERENCES "accounts"("account_id"),
    ADD CONSTRAINT "fk_product_comments_product"
    FOREIGN KEY ("product_id") REFERENCES "products"("product_id");

ALTER TABLE "stocks"
    ADD CONSTRAINT "fk_stocks_warehouse"
        FOREIGN KEY ("warehouse_id") REFERENCES "warehouses"("warehouse_id"),
    ADD CONSTRAINT "fk_stocks_product"
    FOREIGN KEY ("product_id") REFERENCES "products"("product_id");

ALTER TABLE "accounts"
    ADD CONSTRAINT "fk_accounts_address"
        FOREIGN KEY ("address_id") REFERENCES "addresses"("address_id");

ALTER TABLE "products"
    ADD CONSTRAINT "fk_products_category"
        FOREIGN KEY ("category_id") REFERENCES "categories"("category_id");

ALTER TABLE "warehouses"
    ADD CONSTRAINT "fk_warehouses_address"
        FOREIGN KEY ("address_id") REFERENCES "addresses"("address_id");

ALTER TABLE "orders"
    ADD CONSTRAINT "fk_orders_account"
        FOREIGN KEY ("account_id") REFERENCES "accounts"("account_id");