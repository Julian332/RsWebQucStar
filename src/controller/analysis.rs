use crate::contract::uni_graph::{get_user_swaps, Swap};
use crate::controller::{PageParam, PageRes};
use crate::domain::models::{NewTgUser, TgUser};
use crate::openapi::default_resp_docs_with_exam;
use crate::schema::tg_user::address;
use crate::schema::tg_user::dsl::tg_user;
use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use alloy::hex::FromHex;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::k256::elliptic_curve::generic_array::typenum::private::Trim;
use alloy::sol_types::private::SolTypeValue;
use alloy::transports::http::reqwest::Url;
use axum::extract::{Path, State};
use axum::response::Json;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, FromPrimitive};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ops::Sub;
use std::str::FromStr;

pub(crate) fn analysis_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/analysis_addr/:addr",
      get_with(analysis_addr, |x| { x.description("analysis_addr") }),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    .with_state(conn_pool)
}
struct SellSwap {
  pub eth_amount: BigDecimal,
  pub token_amount: BigDecimal,
  pub token_addr: Address,
  pub token_price: BigDecimal,
  pub timestamp: u64,
}

struct BuySwap {
  pub eth_amount: BigDecimal,
  pub token_amount: BigDecimal,
  pub token_addr: Address,
  pub token_price: BigDecimal,
  pub timestamp: u64,
}
pub async fn analysis_addr(
  Path(addr): Path<String>) -> Result<Json<AnalysisResp>, String> {
  let swaps = match get_user_swaps(addr).await {
    Ok(x) => { x }
    Err(e) => {
      println!("{}", e);
      panic!("error on get grpahql")
    }
  };
  //sell "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
  let sell1 = swaps.iter().filter(|x| {
    x.pair.token0.id == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" && x.amount0_in == "0"
  }).map(|x| {
    SellSwap {
      eth_amount: BigDecimal::from_str(&x.amount0_out).unwrap(),
      token_amount: BigDecimal::from_str(&x.amount1_in).unwrap(),
      token_addr: Address::from_hex(&x.pair.token1.id).unwrap(),
      token_price: BigDecimal::from_str(&x.pair.token1_price).unwrap(),
      timestamp: u64::from_str(&x.timestamp).unwrap(),
    }
  });
  let mut sells: Vec<_> = swaps.iter().filter(|x| {
    x.pair.token1.id == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" && x.amount1_in == "0"
  }).map(|x| {
    SellSwap {
      eth_amount: BigDecimal::from_str(&x.amount1_out).unwrap(),
      token_amount: BigDecimal::from_str(&x.amount0_in).unwrap(),
      token_addr: Address::from_hex(&x.pair.token0.id).unwrap(),
      token_price: BigDecimal::from_str(&x.pair.token0_price).unwrap(),
      timestamp: u64::from_str(&x.timestamp).unwrap(),
    }
  }).chain(sell1).collect();
  sells.sort_unstable_by(|l, r| { l.timestamp.cmp(&r.timestamp) });

  let buy1 = swaps.iter().filter(|x| {
    x.pair.token0.id == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" && x.amount0_out == "0"
  }).map(|x| {
    BuySwap {
      eth_amount: BigDecimal::from_str(&x.amount0_in).unwrap(),
      token_amount: BigDecimal::from_str(&x.amount1_out).unwrap(),
      token_addr: Address::from_hex(&x.pair.token1.id).unwrap(),
      token_price: BigDecimal::from_str(&x.pair.token1_price).unwrap(),
      timestamp: u64::from_str(&x.timestamp).unwrap(),

    }
  });
  let mut buys: Vec<_> = swaps.iter().filter(|x| {
    x.pair.token1.id == "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" && x.amount1_out == "0"
  }).map(|x| {
    BuySwap {
      eth_amount: BigDecimal::from_str(&x.amount1_in).unwrap(),
      token_amount: BigDecimal::from_str(&x.amount0_out).unwrap(),
      token_addr: Address::from_hex(&x.pair.token0.id).unwrap(),
      token_price: BigDecimal::from_str(&x.pair.token0_price).unwrap(),
      timestamp: u64::from_str(&x.timestamp).unwrap(),
    }
  }).chain(buy1).collect();
  buys.sort_unstable_by(|l, r| { l.timestamp.cmp(&r.timestamp) });
  // let result = tg_user.find(id_param).select(TgUser::as_select()).first(&mut connection).optional().unwrap();

  let mut sell_eth_sum = BigDecimal::from(0);
  for x in &sells {
    sell_eth_sum += &x.eth_amount;
  }
  let mut buy_eth_sum = BigDecimal::from(0);
  for x in &buys {
    buy_eth_sum += &x.eth_amount;
  }
  let mut sell_map = HashMap::new();
  for sell in sells {
    let x1 = sell_map.entry(sell.token_addr).or_insert(vec![]);
    x1.push(sell);
  }

  let mut buy_map = HashMap::new();
  for buy in buys {
    let x1 = buy_map.entry(buy.token_addr).or_insert(vec![]);
    x1.push(buy);
  }

  // let sell_map :HashMap<_,_> = sell_map.iter().map(|(k, v)| {
  //   if v.last().unwrap().token_price.gt(&v.first().unwrap().token_price) {
  //     (k, (v, true))
  //   } else {
  //     (k, (v, false))
  //   }
  // }).collect();

  let mut tokens: Vec<_> = buy_map.into_iter().map(|(k, v)| {
    let mut sell_sum = BigDecimal::from(0);
    for x in sell_map.get(&k).unwrap_or(&vec![]) {
      sell_sum += &x.eth_amount
    }
    let mut buy_sum = BigDecimal::from(0);

    for x in &v {
      buy_sum += &x.eth_amount
    }

    if v.last().unwrap().token_price.gt(&v.first().unwrap().token_price) {
      (k, (true, sell_sum - buy_sum))
    } else {
      (k, (false, sell_sum - buy_sum))
    }
  }).collect();
  tokens.sort_unstable_by(|l, r| { l.1.1.cmp(&r.1.1) });
  let mut win_count = 0;
  for x in &tokens {
    if x.1.0 {
      win_count += 1;
    }
  }


  Ok(Json(AnalysisResp {
    total_profit: sell_eth_sum.clone() - buy_eth_sum.clone(),
    total_buy: buy_eth_sum,
    total_sell: sell_eth_sum,
    win_rate: BigDecimal::from(win_count).with_scale(6) / BigDecimal::from(tokens.len() as i64),
    tokens: tokens.into_iter().map(|x| { (x.0.to_string(), (x.1.0, x.1.1)) }).collect(),
  }))
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct AnalysisResp {
  total_profit: BigDecimal,
  total_buy: BigDecimal,
  total_sell: BigDecimal,
  win_rate: BigDecimal,
  // top_token: Vec<Address>,
  tokens: Vec<(String, (bool, BigDecimal))>,
}

