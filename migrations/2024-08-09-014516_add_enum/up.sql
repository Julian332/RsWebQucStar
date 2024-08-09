-- Your SQL goes here
CREATE TYPE order_type AS ENUM ('trading', 'pending', 'following');
CREATE TYPE sell_buy AS ENUM ('sell', 'buy');