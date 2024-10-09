-- Your SQL goes here


CREATE TABLE "permissions"
(
    "id"          SERIAL8   NOT NULL PRIMARY KEY,
    "name"        TEXT      NOT NULL,
    "remark"      TEXT      NOT NULL,
    "update_time" TIMESTAMPTZ NOT NULL,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8      NOT NULL,
    "update_by"   INT8      NOT NULL,
    "is_delete"   BOOL      NOT NULL
);

CREATE TABLE "groups"
(
    "id"          SERIAL8   NOT NULL PRIMARY KEY,
    "name"        TEXT      NOT NULL,
    "remark"      TEXT      NOT NULL,
    "update_time" TIMESTAMPTZ NOT NULL,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8      NOT NULL,
    "update_by"   INT8      NOT NULL,
    "is_delete"   BOOL      NOT NULL
);

CREATE TABLE "users"
(
    "id"          SERIAL8   NOT NULL PRIMARY KEY,
    "username"    TEXT      NOT NULL,
    "password"    TEXT      NOT NULL,
    "group_id"    INT8      NOT NULL,
    "tenantry"    TEXT      NOT NULL,
    "remark"      TEXT      NOT NULL,
    "update_time" TIMESTAMPTZ NOT NULL,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8      NOT NULL,
    "update_by"   INT8      NOT NULL,
    "is_delete"   BOOL      NOT NULL,
    FOREIGN KEY ("group_id") REFERENCES "groups" ("id")
);
alter table users
    add constraint uni_name
        unique (username);

CREATE TABLE "groups_permissions"
(
    "group_id"      INT8 NOT NULL,
    "permission_id" INT8 NOT NULL,
    PRIMARY KEY ("group_id", "permission_id")
);

