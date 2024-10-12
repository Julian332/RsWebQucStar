-- Your SQL goes here




CREATE TABLE "auction"(
	"token_addr" TEXT NOT NULL,
	"name" TEXT NOT NULL,
	"symbol" TEXT NOT NULL,
	"once_amount" INT8 NOT NULL,
	"total_supply" NUMERIC NOT NULL,
	"total_eth" NUMERIC NOT NULL,
	"start_time" TIMESTAMPTZ NOT NULL,
	"publish_time" TIMESTAMPTZ NOT NULL,
	"is_burn_lp_token" BOOL NOT NULL,
	"creator_addr" TEXT NOT NULL,
	"creator_id" TEXT NOT NULL,
	"transaction_hash" TEXT NOT NULL,
	"description" TEXT NOT NULL,
	"image" TEXT NOT NULL,
	"id" SERIAL8 NOT NULL PRIMARY KEY,
	"remark" TEXT,
	"update_time" TIMESTAMPTZ,
	"create_time" TIMESTAMPTZ NOT NULL,
	"create_by" INT8 NOT NULL,
	"update_by" INT8,
	"is_delete" BOOL NOT NULL
);

