-- Your SQL goes here

DROP TABLE IF EXISTS "posts";

ALTER TABLE "trading_order" DROP COLUMN "order_type";
ALTER TABLE "trading_order" ADD COLUMN "order_type" VARCHAR NOT NULL;

