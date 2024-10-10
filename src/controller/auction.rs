use crate::models::Auction;
use crate::web_router_gen;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
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
)]
#[diesel(table_name = crate::schema::auction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAuction {
    pub token_addr: String,
    pub name: String,
    pub symbol: String,
    pub once_amount: i64,
    pub total_supply: BigDecimal,
    pub total_eth: BigDecimal,
    pub start_time: DateTime<Utc>,
    pub publish_time: DateTime<Utc>,
    pub is_burn_lp_token: bool,
    pub creator_addr: String,
    pub creator_id: String,
    pub transaction_hash: String,
    pub description: String,
    pub image: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}

web_router_gen!(auction, NewAuction, Auction);
