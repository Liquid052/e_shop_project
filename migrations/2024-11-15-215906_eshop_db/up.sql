-- Enable foreign key support
PRAGMA foreign_keys = ON;

-- Create tables with foreign key constraints inline
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
                          FOREIGN KEY (address_id) REFERENCES addresses (address_id) ON DELETE CASCADE
);

CREATE TABLE products (
                          product_id INTEGER PRIMARY KEY,
                          name TEXT NOT NULL,
                          description TEXT NOT NULL,
                          price REAL NOT NULL,
                          category_id INTEGER NOT NULL,
                          FOREIGN KEY (category_id) REFERENCES categories (category_id) ON DELETE CASCADE
);

CREATE TABLE warehouses (
                            warehouse_id INTEGER PRIMARY KEY,
                            name TEXT NOT NULL,
                            capacity INTEGER NOT NULL,
                            address_id INTEGER NOT NULL,
                            FOREIGN KEY (address_id) REFERENCES addresses (address_id) ON DELETE CASCADE
);

CREATE TABLE orders (
                        order_id INTEGER PRIMARY KEY,
                        total_price REAL NOT NULL,
                        finished BOOLEAN NOT NULL,
                        account_id INTEGER NOT NULL,
                        FOREIGN KEY (account_id) REFERENCES accounts (account_id) ON DELETE CASCADE
);

CREATE TABLE ordered_products (
                                  ordered_product_id INTEGER PRIMARY KEY,
                                  amount INTEGER NOT NULL,
                                  order_id INTEGER NOT NULL,
                                  product_id INTEGER NOT NULL,
                                  FOREIGN KEY (order_id) REFERENCES orders (order_id) ON DELETE CASCADE,
                                  FOREIGN KEY (product_id) REFERENCES products (product_id) ON DELETE CASCADE
);

CREATE TABLE product_comments (
                                  comment_id INTEGER PRIMARY KEY,
                                  content TEXT NOT NULL,
                                  account_id INTEGER NOT NULL,
                                  product_id INTEGER NOT NULL,
                                  FOREIGN KEY (account_id) REFERENCES accounts (account_id) ON DELETE CASCADE,
                                  FOREIGN KEY (product_id) REFERENCES products (product_id) ON DELETE CASCADE
);

CREATE TABLE stocks (
                        stock_id INTEGER PRIMARY KEY,
                        amount INTEGER NOT NULL,
                        warehouse_id INTEGER NOT NULL,
                        product_id INTEGER NOT NULL,
                        FOREIGN KEY (warehouse_id) REFERENCES warehouses (warehouse_id) ON DELETE CASCADE,
                        FOREIGN KEY (product_id) REFERENCES products (product_id) ON DELETE CASCADE
);

-- Insert sample data
INSERT INTO addresses (address_id, street, city, state, postal_code, country) VALUES
                                                                                  (1, '123 Main St', 'New York', 'NY', '10001', 'USA'),
                                                                                  (2, '456 Oak Ave', 'Los Angeles', 'CA', '90001', 'USA'),
                                                                                  (3, '789 Pine Rd', 'Chicago', 'IL', '60601', 'USA'),
                                                                                  (4, '321 Maple Dr', 'Houston', 'TX', '77001', 'USA'),
                                                                                  (5, '654 Cedar Ln', 'Phoenix', 'AZ', '85001', 'USA');

INSERT INTO categories (category_id, type_name) VALUES (1, 'Womens Clothing'),
                                                       (2, 'Mens Clothing'),
                                                       (3, 'Accessories'),
                                                       (4, 'Shoes'),
                                                       (5, 'Bags');

INSERT INTO accounts (account_id, username, role, first_name, last_name, phone_number, email, credit_card, address_id) VALUES
                                                                                                                           (1, 'john_doe', 'customer', 'John', 'Doe', '555-0123', 'john@example.com', '4532-xxxx-xxxx-1234', 1),
                                                                                                                           (2, 'jane_smith', 'customer', 'Jane', 'Smith', '555-0124', 'jane@example.com', '4532-xxxx-xxxx-5678', 2),
                                                                                                                           (3, 'bob_wilson', 'admin', 'Bob', 'Wilson', '555-0125', 'bob@example.com', '4532-xxxx-xxxx-9012', 3);
INSERT INTO products (product_id, name, description, price, category_id) VALUES (1, 'Designer Silk Dress',
                                                                                 'Elegant silk evening dress with floral pattern',
                                                                                 299.99, 1),
                                                                                (2, 'Classic Mens Suit',
                                                                                 'Two-piece wool blend suit in navy blue',
                                                                                 599.99, 2),
                                                                                (3, 'Leather Crossbody Bag',
                                                                                 'Genuine leather bag with adjustable strap',
                                                                                 149.99, 5),
                                                                                (4, 'Designer Sneakers',
                                                                                 'Limited edition designer sneakers',
                                                                                 199.99, 4),
                                                                                (5, 'Gold Plated Necklace',
                                                                                 'Delicate chain with pendant', 79.99,
                                                                                 3);

INSERT INTO warehouses (warehouse_id, name, capacity, address_id) VALUES
                                                                      (1, 'East Coast Facility', 10000, 4),
                                                                      (2, 'West Coast Facility', 15000, 5);


INSERT INTO orders (order_id, total_price, finished, account_id) VALUES (1, 449.98, 1, 1), -- Dress + Necklace
                                                                        (2, 749.98, 0, 2), -- Suit + Bag
                                                                        (3, 199.99, 1, 3);
-- Sneakers

-- Updated ordered products
INSERT INTO ordered_products (ordered_product_id, amount, order_id, product_id) VALUES (1, 1, 1, 1), -- Dress
                                                                                       (2, 1, 1, 5), -- Necklace
                                                                                       (3, 1, 2, 2), -- Suit
                                                                                       (4, 1, 2, 3), -- Bag
                                                                                       (5, 1, 3, 4);
-- Sneakers

-- Updated product comments
INSERT INTO product_comments (comment_id, content, account_id, product_id) VALUES (1,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (2,
                                                                                   'Excellent quality suit, worth every penny',
                                                                                   2, 2),
                                                                                  (3,
                                                                                   'The leather is so soft and the size is perfect',
                                                                                   3, 3),
                                                                                  (4,
                                                                                   'These sneakers are amazing and so comfortable',
                                                                                   1, 4),
                                                                                  (5,
                                                                                   'Beautiful necklace, looks even better in person',
                                                                                   2, 5),
                                                                                  (6,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (7,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (8,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (9,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (10,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (11,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (12,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1),
                                                                                  (13,
                                                                                   'Perfect fit and beautiful fabric!',
                                                                                   1, 1);

-- Updated stock information
INSERT INTO stocks (stock_id, amount, warehouse_id, product_id) VALUES (1, 25, 1, 1), -- Dresses in East Coast
                                                                       (2, 30, 1, 2), -- Suits in East Coast
                                                                       (3, 50, 2, 3), -- Bags in West Coast
                                                                       (4, 45, 2, 4), -- Sneakers in West Coast
                                                                       (5, 75, 1, 5); -- Necklaces in East Coast