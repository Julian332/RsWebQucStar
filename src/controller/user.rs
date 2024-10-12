use crate::models::User;
use crate::models::UserBuilder;
use crate::web_router_gen;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use diesel::sql_types::Text;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    Builder,
)]
#[builder(derive(Deserialize, Serialize, JsonSchema))]
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

#[derive(Serialize, Deserialize, JsonSchema, Builder)]
#[builder(derive(Deserialize, Serialize, JsonSchema))]
pub struct NewUserParam {
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
}
web_router_gen!(users, NewUser, User, UserBuilder);
