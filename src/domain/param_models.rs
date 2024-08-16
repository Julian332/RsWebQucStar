// #[macro_use] extern crate diesel;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(
  Queryable,
  Debug,
  Serialize,
  Deserialize,
  JsonSchema,
  Insertable,
  Selectable,
  AsChangeset,
  Clone
)]
#[diesel(table_name = crate::schema::trading_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTradingOrder {
  pub sell_or_buy: SellBuy,
  pub target_token: String,
  pub from_token: String,
  // pub trading_uer: i64,
  pub boost_mode: bool,
  pub mev_protected: bool,
  pub priority_fee: Option<BigDecimal>,

  // pub target_amount: Option<BigDecimal>,
  pub from_token_amount: BigDecimal,
  // pub pending_target_price: Option<BigDecimal>,
  // pub expire_at: Option<DateTime<Utc>>,
  pub order_type: OrderType,
  pub slippage: Option<BigDecimal>,
  pub user_addr: String,

}

#[derive(
  Queryable,
  Debug,
  Serialize,
  Deserialize,
  JsonSchema,
  Insertable,
  Selectable,
  AsChangeset,
  Clone
)]
#[diesel(table_name = crate::schema::trading_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewTradingOrderWithUserId {
  pub sell_or_buy: SellBuy,
  pub target_token: String,
  pub from_token: String,
  pub trading_uer: i64,
  pub boost_mode: bool,
  pub mev_protected: bool,
  pub priority_fee: Option<BigDecimal>,

  // pub target_amount: Option<BigDecimal>,
  pub from_token_amount: BigDecimal,
  // pub pending_target_price: Option<BigDecimal>,
  // pub expire_at: Option<DateTime<Utc>>,
  pub order_type: OrderType,
  pub slippage: Option<BigDecimal>,
  pub user_addr: String,

}




#[derive(
  Debug,
  Serialize,
  Deserialize,
  JsonSchema,
  Clone
)]
#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SellBuy"]
pub enum SellBuy {
  Sell,
  Buy,
}
#[derive(
  Debug,
  Serialize,
  Deserialize,
  JsonSchema,
  Clone
)]
#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::OrderType"]
pub enum OrderType {
  Trading,
  Pending,
  Following,
}

#[derive(Debug)]
#[derive(Queryable,Insertable,Serialize,Deserialize,JsonSchema,Clone)]
#[diesel(table_name = crate::schema::addr_subscribes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAddrSubscribes {
  // pub id: i64,
  // pub deleted: bool,
  // pub create_time: DateTime<Utc>,
  // pub update_time: Option<DateTime<Utc>>,
  pub following_addr: String,
  pub subscribers: Option<Vec<Option<String>>>,
}