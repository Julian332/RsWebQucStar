-- This file should undo anything in `up.sql`
ALTER TABLE "auction" DROP COLUMN "is_published";
ALTER TABLE "auction" DROP COLUMN "published_price_in_wei";
ALTER TABLE "auction" DROP COLUMN "latest_price_in_wei";





