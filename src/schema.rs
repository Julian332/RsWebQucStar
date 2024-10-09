// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int8,
        name -> Text,
        remark -> Text,
        update_time -> Timestamptz,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Int8,
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
        remark -> Text,
        update_time -> Timestamptz,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Int8,
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
        remark -> Text,
        update_time -> Timestamptz,
        create_time -> Timestamptz,
        create_by -> Int8,
        update_by -> Int8,
        is_delete -> Bool,
    }
}

diesel::joinable!(users -> groups (group_id));
diesel::joinable!(groups_permissions -> groups (group_id));
diesel::joinable!(groups_permissions -> permissions (permission_id));

diesel::allow_tables_to_appear_in_same_query!(groups, groups_permissions, permissions, users,);
