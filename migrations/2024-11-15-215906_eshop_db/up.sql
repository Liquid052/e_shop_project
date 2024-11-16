-- Enable foreign key support
PRAGMA foreign_keys = ON;

-- Create tables with foreign key constraints inline
CREATE TABLE categories (
                            category_id INTEGER PRIMARY KEY,
                            type_name TEXT NOT NULL
);

CREATE TABLE addresses (
                           address_id INTEGER PRIMARY KEY,
                           street TEXT NOT NULL,
                           city TEXT NOT NULL,
                           state TEXT NOT NULL,
                           postal_code TEXT NOT NULL,
                           country TEXT NOT NULL
);

CREATE TABLE accounts (
                          account_id INTEGER PRIMARY KEY,
                          username TEXT NOT NULL,
                          role TEXT NOT NULL,
                          first_name TEXT NOT NULL,
                          last_name TEXT NOT NULL,
                          phone_number TEXT NOT NULL,
                          email TEXT NOT NULL,
                          credit_card TEXT NOT NULL,
                          address_id INTEGER NOT NULL,
                          FOREIGN KEY (address_id) REFERENCES addresses(address_id)
);

CREATE TABLE products (
                          product_id INTEGER PRIMARY KEY,
                          name TEXT NOT NULL,
                          description TEXT NOT NULL,
                          price REAL NOT NULL,
                          category_id INTEGER NOT NULL,
                          FOREIGN KEY (category_id) REFERENCES categories(category_id)
);

CREATE TABLE warehouses (
                            warehouse_id INTEGER PRIMARY KEY,
                            name TEXT NOT NULL,
                            capacity INTEGER NOT NULL,
                            address_id INTEGER NOT NULL,
                            FOREIGN KEY (address_id) REFERENCES addresses(address_id)
);

CREATE TABLE orders (
                        order_id INTEGER PRIMARY KEY,
                        total_price REAL NOT NULL,
                        finished BOOLEAN NOT NULL,
                        account_id INTEGER NOT NULL,
                        FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

CREATE TABLE ordered_products (
                                  ordered_product_id INTEGER PRIMARY KEY,
                                  amount INTEGER NOT NULL,
                                  order_id INTEGER NOT NULL,
                                  product_id INTEGER NOT NULL,
                                  FOREIGN KEY (order_id) REFERENCES orders(order_id),
                                  FOREIGN KEY (product_id) REFERENCES products(product_id)
);

CREATE TABLE product_comments (
                                  comment_id INTEGER PRIMARY KEY,
                                  content TEXT NOT NULL,
                                  account_id INTEGER NOT NULL,
                                  product_id INTEGER NOT NULL,
                                  FOREIGN KEY (account_id) REFERENCES accounts(account_id),
                                  FOREIGN KEY (product_id) REFERENCES products(product_id)
);

CREATE TABLE stocks (
                        stock_id INTEGER PRIMARY KEY,
                        amount INTEGER NOT NULL,
                        warehouse_id INTEGER NOT NULL,
                        product_id INTEGER NOT NULL,
                        FOREIGN KEY (warehouse_id) REFERENCES warehouses(warehouse_id),
                        FOREIGN KEY (product_id) REFERENCES products(product_id)
);

-- Insert sample data
INSERT INTO addresses (address_id, street, city, state, postal_code, country) VALUES
                                                                                  (1, '123 Main St', 'New York', 'NY', '10001', 'USA'),
                                                                                  (2, '456 Oak Ave', 'Los Angeles', 'CA', '90001', 'USA'),
                                                                                  (3, '789 Pine Rd', 'Chicago', 'IL', '60601', 'USA'),
                                                                                  (4, '321 Maple Dr', 'Houston', 'TX', '77001', 'USA'),
                                                                                  (5, '654 Cedar Ln', 'Phoenix', 'AZ', '85001', 'USA');

INSERT INTO categories (category_id, type_name) VALUES
                                                    (1, 'Electronics'),
                                                    (2, 'Clothing'),
                                                    (3, 'Books'),
                                                    (4, 'Home & Garden'),
                                                    (5, 'Sports & Outdoors');

INSERT INTO accounts (account_id, username, role, first_name, last_name, phone_number, email, credit_card, address_id) VALUES
                                                                                                                           (1, 'john_doe', 'customer', 'John', 'Doe', '555-0123', 'john@example.com', '4532-xxxx-xxxx-1234', 1),
                                                                                                                           (2, 'jane_smith', 'customer', 'Jane', 'Smith', '555-0124', 'jane@example.com', '4532-xxxx-xxxx-5678', 2),
                                                                                                                           (3, 'bob_wilson', 'admin', 'Bob', 'Wilson', '555-0125', 'bob@example.com', '4532-xxxx-xxxx-9012', 3);

INSERT INTO products (product_id, name, description, price, category_id) VALUES
                                                                             (1, 'Laptop Pro', 'High-performance laptop with SSD', 1299.99, 1),
                                                                             (2, 'Cotton T-Shirt', 'Comfortable casual wear', 19.99, 2),
                                                                             (3, 'Programming Guide', 'Complete guide to coding', 49.99, 3),
                                                                             (4, 'Garden Tools Set', 'Essential gardening tools', 89.99, 4),
                                                                             (5, 'Tennis Racket', 'Professional grade racket', 159.99, 5);

INSERT INTO warehouses (warehouse_id, name, capacity, address_id) VALUES
                                                                      (1, 'East Coast Facility', 10000, 4),
                                                                      (2, 'West Coast Facility', 15000, 5);

INSERT INTO orders (order_id, total_price, finished, account_id) VALUES
                                                                     (1, 1349.98, 1, 1),
                                                                     (2, 269.97, 0, 2),
                                                                     (3, 89.99, 1, 3);

INSERT INTO ordered_products (ordered_product_id, amount, order_id, product_id) VALUES
                                                                                    (1, 1, 1, 1),
                                                                                    (2, 2, 1, 2),
                                                                                    (3, 1, 2, 3),
                                                                                    (4, 1, 2, 4),
                                                                                    (5, 1, 3, 4);

INSERT INTO product_comments (comment_id, content, account_id, product_id) VALUES
                                                                               (1, 'Great laptop, very fast!', 1, 1),
                                                                               (2, 'Comfortable fit, would buy again', 2, 2),
                                                                               (3, 'Very informative book', 3, 3),
                                                                               (4, 'Tools are durable and well-made', 1, 4),
                                                                               (5, 'Perfect for my tennis games', 2, 5);

INSERT INTO stocks (stock_id, amount, warehouse_id, product_id) VALUES
                                                                    (1, 50, 1, 1),
                                                                    (2, 200, 1, 2),
                                                                    (3, 100, 2, 3),
                                                                    (4, 75, 2, 4),
                                                                    (5, 25, 1, 5);