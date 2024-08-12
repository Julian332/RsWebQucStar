-- Your SQL goes here
alter table public.trading_order
    alter column from_token_amount set not null;

