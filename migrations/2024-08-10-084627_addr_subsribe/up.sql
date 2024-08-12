-- Your SQL goes here


-- ALTER TABLE "trading_order" DROP COLUMN "sell_or_buy";
-- ALTER TABLE "trading_order" DROP COLUMN "order_type";
-- ALTER TABLE "trading_order" ADD COLUMN "sell_or_buy" SELLBUY NOT NULL;
-- ALTER TABLE "trading_order" ADD COLUMN "order_type" ORDERTYPE NOT NULL;

CREATE TABLE "addr_subscribes"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"deleted" BOOL NOT NULL,
	"create_time" TIMESTAMPTZ NOT NULL,
	"update_time" TIMESTAMPTZ,
	"addr" VARCHAR NOT NULL,
	"subscribers" text[] NOT NULL
);

