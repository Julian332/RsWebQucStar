use chrono::{DateTime, Utc};
use derive_builder::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Builder)]
#[builder(derive(Deserialize, Serialize, JsonSchema))]
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
