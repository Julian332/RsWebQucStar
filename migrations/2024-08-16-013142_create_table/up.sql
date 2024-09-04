-- Your SQL goes here


create type order_type as enum ( 'trading', 'pending', 'following');
alter type order_type owner to postgres;

create type sell_buy as enum ('sell', 'buy');

alter type sell_buy owner to postgres;


-- auto-generated definition
CREATE TABLE "following_order"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"deleted" BOOL NOT NULL,
	"create_time" TIMESTAMPTZ NOT NULL,
	"update_time" TIMESTAMPTZ
);

CREATE TABLE "trading_order"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"deleted" BOOL NOT NULL,
	"create_time" TIMESTAMPTZ NOT NULL,
	"update_time" TIMESTAMPTZ,
	"sell_or_buy" sell_buy NOT NULL,
	"target_token" VARCHAR NOT NULL,
	"from_token" VARCHAR NOT NULL,
	"trading_uer" INT8 NOT NULL,
	"boost_mode" BOOL NOT NULL,
	"mev_protected" BOOL NOT NULL,
	"priority_fee" NUMERIC,
	"is_succeed" BOOL,
	"tx_hash" VARCHAR,
	"tx_receipt" JSONB,
	"target_amount" NUMERIC,
	"from_token_amount" NUMERIC NOT NULL,
	"pending_target_price" NUMERIC,
	"expire_at" TIMESTAMPTZ,
	"fee" NUMERIC,
	"order_type" order_type NOT NULL,
	"slippage" NUMERIC,
	"user_addr" VARCHAR NOT NULL
);

CREATE TABLE "addr_subscribes"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"deleted" BOOL NOT NULL,
	"create_time" TIMESTAMPTZ NOT NULL,
	"update_time" TIMESTAMPTZ,
	"following_addr" VARCHAR NOT NULL,
	"subscribers" TEXT[]
);

CREATE TABLE "tg_user"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"deleted" BOOL NOT NULL,
	"create_time" TIMESTAMPTZ NOT NULL,
	"update_time" TIMESTAMPTZ,
	"address" VARCHAR NOT NULL,
	"private_key" VARCHAR,
	"fee_staged" NUMERIC,
	"fee_received" NUMERIC,
	"parent" VARCHAR
);

