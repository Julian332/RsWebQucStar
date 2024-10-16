use chrono::{DateTime, Utc};
use derive_builder::WebApiGen;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub is_delete: bool,
}
#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    AsChangeset,
    Debug,
    WebApiGen,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}
