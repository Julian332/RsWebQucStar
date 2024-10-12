// #[macro_use] extern crate diesel;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, diesel_derive_enum::DbEnum)]
// #[ExistingTypePath = "crate::schema::sql_types::SellBuy"]
// pub enum SellBuy {
//     Sell,
//     Buy,
// }
// #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, diesel_derive_enum::DbEnum)]
// #[ExistingTypePath = "crate::schema::sql_types::OrderType"]
// pub enum OrderType {
//     Trading,
//     Pending,
//     Following,
// }
