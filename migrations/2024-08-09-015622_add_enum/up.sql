-- Your SQL goes here
alter table public.trading_order
    alter column sell_or_buy type sell_buy using sell_or_buy::sell_buy;

alter table public.trading_order
    alter column order_type type order_type using order_type::order_type;



