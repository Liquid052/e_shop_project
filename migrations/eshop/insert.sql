INSERT INTO "addresses" ("address_id", "street", "city", "state", "postal_code", "country") VALUES
                                                                                                (1, '123 Main St', 'New York', 'NY', '10001', 'USA'),
                                                                                                (2, '456 Oak Ave', 'Los Angeles', 'CA', '90001', 'USA'),
                                                                                                (3, '789 Pine Rd', 'Chicago', 'IL', '60601', 'USA'),
                                                                                                (4, '321 Maple Dr', 'Houston', 'TX', '77001', 'USA'),
                                                                                                (5, '654 Cedar Ln', 'Phoenix', 'AZ', '85001', 'USA');

-- Insert categories as they're referenced by products
INSERT INTO "categories" ("category_id", "type_name") VALUES
                                                          (1, 'Electronics'),
                                                          (2, 'Clothing'),
                                                          (3, 'Books'),
                                                          (4, 'Home & Garden'),
                                                          (5, 'Sports & Outdoors');

-- Insert accounts as they're referenced by orders and comments
INSERT INTO "accounts" ("account_id", "username", "role", "first_name", "last_name", "phone_number",
                        "email", "balance", "credit_card", "address_id") VALUES
                                                                             (1, 'john_doe', 'customer', 'John', 'Doe', '555-0123', 'john@example.com', 1000.00,
                                                                              '4532-xxxx-xxxx-1234', 1),
                                                                             (2, 'jane_smith', 'customer', 'Jane', 'Smith', '555-0124', 'jane@example.com', 2500.00,
                                                                              '4532-xxxx-xxxx-5678', 2),
                                                                             (3, 'bob_wilson', 'admin', 'Bob', 'Wilson', '555-0125', 'bob@example.com', 500.00,
                                                                              '4532-xxxx-xxxx-9012', 3);

-- Insert products as they're referenced by orders and stocks
INSERT INTO "products" ("product_id", "name", "description", "price", "category_id") VALUES
                                                                                         (1, 'Laptop Pro', 'High-performance laptop with SSD', 1299.99, 1),
                                                                                         (2, 'Cotton T-Shirt', 'Comfortable casual wear', 19.99, 2),
                                                                                         (3, 'Programming Guide', 'Complete guide to coding', 49.99, 3),
                                                                                         (4, 'Garden Tools Set', 'Essential gardening tools', 89.99, 4),
                                                                                         (5, 'Tennis Racket', 'Professional grade racket', 159.99, 5);

-- Insert warehouses as they're referenced by stocks
INSERT INTO "warehouses" ("warehouse_id", "name", "capacity", "address_id") VALUES
                                                                                (1, 'East Coast Facility', 10000, 4),
                                                                                (2, 'West Coast Facility', 15000, 5);

-- Insert orders as they're referenced by ordered_products
INSERT INTO "orders" ("order_id", "total_price", "finished", "account_id") VALUES
                                                                               (1, 1349.98, true, 1),
                                                                               (2, 269.97, false, 2),
                                                                               (3, 89.99, true, 3);

-- Insert ordered_products
INSERT INTO "ordered_products" ("ordered_product_id", "amount", "order_id", "product_id") VALUES
                                                                                              (1, 1, 1, 1),
                                                                                              (2, 2, 1, 2),
                                                                                              (3, 1, 2, 3),
                                                                                              (4, 1, 2, 4),
                                                                                              (5, 1, 3, 4);

-- Insert product_comments
INSERT INTO "product_comments" ("comment_id", "content", "account_id", "product_id") VALUES
                                                                                         (1, 'Great laptop, very fast!', 1, 1),
                                                                                         (2, 'Comfortable fit, would buy again', 2, 2),
                                                                                         (3, 'Very informative book', 3, 3),
                                                                                         (4, 'Tools are durable and well-made', 1, 4),
                                                                                         (5, 'Perfect for my tennis games', 2, 5);

-- Insert stocks
INSERT INTO "stocks" ("stock_id", "amount", "warehouse_id", "product_id") VALUES
                                                                              (1, 50, 1, 1),
                                                                              (2, 200, 1, 2),
                                                                              (3, 100, 2, 3),
                                                                              (4, 75, 2, 4),
                                                                              (5, 25, 1, 5);