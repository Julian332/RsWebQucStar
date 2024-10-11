-- Your SQL goes here
ALTER TABLE "auction" ADD COLUMN "is_published" BOOL NOT NULL;
ALTER TABLE "auction" ADD COLUMN "published_price_in_wei" NUMERIC;
ALTER TABLE "auction" ADD COLUMN "latest_price_in_wei" NUMERIC;





