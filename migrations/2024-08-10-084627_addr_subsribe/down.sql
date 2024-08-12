-- This file should undo anything in `up.sql`


-- ALTER TABLE "trading_order" DROP COLUMN "sell_or_buy";
-- ALTER TABLE "trading_order" DROP COLUMN "order_type";
-- ALTER TABLE "trading_order" ADD COLUMN "sell_or_buy" SELL_BUY NOT NULL;
-- ALTER TABLE "trading_order" ADD COLUMN "order_type" ORDER_TYPE NOT NULL;

DROP TABLE IF EXISTS "addr_subscribes";
