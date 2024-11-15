-- First drop all foreign key constraints
ALTER TABLE "ordered_products"
DROP CONSTRAINT IF EXISTS "fk_ordered_products_order",
    DROP CONSTRAINT IF EXISTS "fk_ordered_products_product";

ALTER TABLE "product_comments"
DROP CONSTRAINT IF EXISTS "fk_product_comments_account",
    DROP CONSTRAINT IF EXISTS "fk_product_comments_product";

ALTER TABLE "stocks"
DROP CONSTRAINT IF EXISTS "fk_stocks_warehouse",
    DROP CONSTRAINT IF EXISTS "fk_stocks_product";

ALTER TABLE "accounts"
DROP CONSTRAINT IF EXISTS "fk_accounts_address";

ALTER TABLE "products"
DROP CONSTRAINT IF EXISTS "fk_products_category";

ALTER TABLE "warehouses"
DROP CONSTRAINT IF EXISTS "fk_warehouses_address";

ALTER TABLE "orders"
DROP CONSTRAINT IF EXISTS "fk_orders_account";

-- Then drop all tables
DROP TABLE IF EXISTS "ordered_products";
DROP TABLE IF EXISTS "product_comments";
DROP TABLE IF EXISTS "stocks";
DROP TABLE IF EXISTS "accounts";
DROP TABLE IF EXISTS "products";
DROP TABLE IF EXISTS "addresses";
DROP TABLE IF EXISTS "warehouses";
DROP TABLE IF EXISTS "orders";
DROP TABLE IF EXISTS "categories";