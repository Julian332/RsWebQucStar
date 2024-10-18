-- Your SQL goes here


CREATE TABLE "permissions"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "name"        TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL
);

CREATE TABLE "groups"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "name"        TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL
);

CREATE TABLE "users"
(
    "id"          SERIAL8     NOT NULL PRIMARY KEY,
    "username"    TEXT        NOT NULL,
    "password"    TEXT        NOT NULL,
    "group_id"    INT8        NOT NULL,
    "tenantry"    TEXT        NOT NULL,
    "remark"      TEXT,
    "update_time" TIMESTAMPTZ,
    "create_time" TIMESTAMPTZ NOT NULL,
    "create_by"   INT8        NOT NULL,
    "update_by"   INT8,
    "is_delete"   BOOL        NOT NULL,
    FOREIGN KEY ("group_id") REFERENCES "groups" ("id")
);
alter table users
    add constraint uni_name
        unique (username);

comment on column users.password is 'password  hash or signature hash';

CREATE TABLE "groups_permissions"
(
    "group_id"      INT8 NOT NULL,
    "permission_id" INT8 NOT NULL,
    PRIMARY KEY ("group_id", "permission_id")
);

alter table groups_permissions
    add constraint groups_permissions_groups_id_fk
        foreign key (group_id) references groups;

alter table groups_permissions
    add constraint groups_permissions_permissions_id_fk
        foreign key (permission_id) references permissions;

CREATE TYPE order_type AS ENUM ('trading', 'pending', 'following');
CREATE TYPE sell_buy AS ENUM ('sell', 'buy');



INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-1, 'common_user', null, null, now(), -2, null, false);

INSERT INTO groups (id, name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES (-2, 'super_admin', null, null, now(), -2, null, false);



INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('common_read', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('common_add', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('common_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('common_update', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('permissions_read', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('permissions_add', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('permissions_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('permissions_update', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('groups_read', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('groups_add', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('groups_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('groups_update', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('users_read', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('users_add', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('users_delete', null, null, now(), -2, null, false);

INSERT INTO permissions (name, remark, update_time, create_time, create_by, update_by, is_delete)
VALUES ('users_update', null, null, now(), -2, null, false);



INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-1, 1);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, 1);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, 2);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, 3);
INSERT INTO groups_permissions (group_id, permission_id)
VALUES (-2, 4);



INSERT INTO users (id, username, password, group_id, tenantry, remark, update_time, create_time, create_by, update_by,
                   is_delete)
VALUES (-1, 'common_user',
        '$argon2id$v=19$m=19456,t=2,p=1$pHJK4Msog1E+V7R4++t+Zg$QnzTOC3JNu50cn0fJcdO5P33WnUUeQRK3oa9M054nko', -1,
        'default', null, null, now(), -2, null, false);

INSERT INTO users (id, username, password, group_id, tenantry, remark, update_time, create_time, create_by, update_by,
                   is_delete)
VALUES (-2, 'super_admin',
        '$argon2id$v=19$m=19456,t=2,p=1$pHJK4Msog1E+V7R4++t+Zg$QnzTOC3JNu50cn0fJcdO5P33WnUUeQRK3oa9M054nko', -2,
        'default', null, null, now(), -2, null, false)
