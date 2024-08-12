-- This file should undo anything in `up.sql`
alter table public.addr_subscribes
    rename column token_addr to addr;

