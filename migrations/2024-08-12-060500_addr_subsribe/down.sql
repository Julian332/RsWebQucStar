-- This file should undo anything in `up.sql`
ALTER TABLE "addr_subscribes" DROP COLUMN "following_addr";
ALTER TABLE "addr_subscribes" DROP COLUMN "subscribers";
ALTER TABLE "addr_subscribes" ADD COLUMN "token_addr" VARCHAR NOT NULL;
ALTER TABLE "addr_subscribes" ADD COLUMN "subscribers" TEXT NOT NULL[];



ALTER TABLE "trading_order" DROP COLUMN "sell_or_buy";
ALTER TABLE "trading_order" DROP COLUMN "order_type";
ALTER TABLE "trading_order" ADD COLUMN "sell_or_buy" SELL_BUY NOT NULL;
ALTER TABLE "trading_order" ADD COLUMN "order_type" ORDER_TYPE NOT NULL;

