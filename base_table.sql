create table tg_user
(
    id          bigserial
        constraint tg_user_pk
            primary key,
    deleted     boolean                  default false             not null,
    create_time timestamp with time zone default CURRENT_TIMESTAMP not null,
    update_time timestamp with time zone
);


CREATE TRIGGER "update_time" BEFORE UPDATE ON "tg_user"
    FOR EACH STATEMENT
    EXECUTE PROCEDURE "trigger_set_timestamp"();


alter table tg_user
    owner to postgres;