// @generated automatically by Diesel CLI.

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
    trading_order (id) {
        id -> Int8,
        deleted -> Bool,
        create_time -> Timestamptz,
        update_time -> Nullable<Timestamptz>,
        sell_or_buy -> Varchar,
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
        from_token_amount -> Nullable<Numeric>,
        pending_target_price -> Nullable<Numeric>,
        expire_at -> Nullable<Timestamptz>,
        fee -> Nullable<Numeric>,
        order_type -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    following_order,
    tg_user,
    trading_order,
);
