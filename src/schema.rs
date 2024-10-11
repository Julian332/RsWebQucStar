// @generated automatically by Diesel CLI.

diesel::table! {
    auction (id) {
        token_addr -> Text,
        name -> Text,
        symbol -> Text,
        once_amount -> Int8,
        total_supply -> Numeric,
        total_eth -> Numeric,
        start_time -> Timestamptz,
        publish_time -> Timestamptz,
        is_burn_lp_token -> Bool,
        creator_addr -> Text,
        creator_id -> Text,
        transaction_hash -> Text,
        description -> Text,
        image -> Text,
        id -> Int8,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
        is_published -> Bool,
        published_price_in_wei -> Nullable<Numeric>,
        latest_price_in_wei -> Nullable<Numeric>,
    }
}

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    groups_permissions (group_id, permission_id) {
        group_id -> Int8,
        permission_id -> Int8,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int8,
        name -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        group_id -> Int8,
        tenantry -> Text,
        remark -> Nullable<Text>,
        update_time -> Nullable<Timestamptz>,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Nullable<Int8>,
        is_delete -> Bool,
    }
}

diesel::joinable!(groups_permissions -> groups (group_id));
diesel::joinable!(groups_permissions -> permissions (permission_id));
diesel::joinable!(users -> groups (group_id));

diesel::allow_tables_to_appear_in_same_query!(
    auction,
    groups,
    groups_permissions,
    permissions,
    users,
);
