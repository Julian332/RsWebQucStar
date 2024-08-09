-- Your SQL goes here
CREATE TYPE order_type AS ENUM ('Trading', 'Pending', 'Following');
CREATE TYPE sell_buy AS ENUM ('Sell', 'Buy');