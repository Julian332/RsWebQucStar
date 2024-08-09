// @generated automatically by Diesel CLI.

pub mod sql_types {
  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "order_type"))]
  pub struct OrderType;

  #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
  #[diesel(postgres_type(name = "sell_buy"))]
  pub struct SellBuy;
}

diesel::table! {
    following_order (id) {
        id -> Int8,
        deleted -> Bool,
        create_time -> Timestamptz,
        update_time -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tg_user (id) {
        id -> Int8,
        deleted -> Bool,
        create_time -> Timestamptz,
        update_time -> Nullable<Timestamptz>,
        address -> Varchar,
        private_key -> Nullable<Varchar>,
        fee_staged -> Nullable<Numeric>,
        fee_received -> Nullable<Numeric>,
        parent -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SellBuy;
    use super::sql_types::OrderType;

    trading_order (id) {
        id -> Int8,
        deleted -> Bool,
        create_time -> Timestamptz,
        update_time -> Nullable<Timestamptz>,
        sell_or_buy -> SellBuy,
        target_token -> Varchar,
        from_token -> Varchar,
        trading_uer -> Int8,
        boost_mode -> Bool,
        mev_protected -> Bool,
        priority_fee -> Nullable<Numeric>,
        is_succeed -> Nullable<Bool>,
        tx_hash -> Nullable<Varchar>,
        tx_receipt -> Nullable<Jsonb>,
        target_amount -> Nullable<Numeric>,
        from_token_amount -> Numeric,
        pending_target_price -> Nullable<Numeric>,
        expire_at -> Nullable<Timestamptz>,
        fee -> Nullable<Numeric>,
        order_type -> OrderType,
        slippage -> Nullable<Numeric>,
        user_addr -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    following_order,
    tg_user,
    trading_order,
);
