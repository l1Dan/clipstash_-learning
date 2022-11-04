-- Add migration script here
CREATE TABLE IF NOT EXISTS clips (
    clip_id TEXT PRIMARY KEY NOT NULL,
    short_code TEXT UNIQUE NOT NULL,
    content TEXT NOT NULL,
    title TEXT,
    posted DATETIME NOT NULL,
    expires DATETIME,
    password TEXT,
    hits BIGINT NOT NULL
);
-- CREATE TABLE IF NOT EXISTS Customers (
--     customer_id TEXT PRIMARY KEY,
--     name TEXT UNIQUE NOT NULL
-- );
-- CREATE TABLE IF NOT EXISTS Transaction (
--     transaction_id TEXT PRIMARY KEY,
--     customer_id TEXT,
--     amount TEXT
-- );
-- INSERT INTO Customers (customer_id, name)
-- VALUES (1, 'Kim');
-- INSERT INTO Customers (customer_id, name)
-- VALUES (2, 'Dave');
-- INSERT INTO Customers (customer_id, name)
-- VALUES (3, 'Tonya');
-- INSERT INTO Transaction (transaction_id, customer_id, amount)
-- VALUES (910, '3', '14.00');
-- INSERT INTO Transaction (transaction_id, customer_id, amount)
-- VALUES (545, '2', '12.00');
-- INSERT INTO Transaction (transaction_id, customer_id, amount)
-- VALUES (746, '3', '33.00');