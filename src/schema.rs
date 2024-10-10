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

diesel::joinable!(users -> groups (group_id));
diesel::joinable!(groups_permissions -> groups (group_id));
diesel::joinable!(groups_permissions -> permissions (permission_id));

diesel::allow_tables_to_appear_in_same_query!(groups, groups_permissions, permissions, users,);
