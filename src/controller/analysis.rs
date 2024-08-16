use crate::contract::uni_graph::get_user_swaps;
use crate::controller::{PageParam, PageRes};
use crate::models::{AddrSubscribes, NewTgUser, TgUser};

use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use alloy::hex::FromHex;
use alloy::primitives::Address;

use crate::controller::analysis::IUniswapV2Pair::{IUniswapV2PairEvents, Swap};
use crate::domain::param_models::NewAddrSubscribes;
use crate::schema;
use crate::schema::addr_subscribes::dsl::addr_subscribes;
use crate::schema::addr_subscribes::{deleted, following_addr};
use alloy::eips::BlockNumberOrTag;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol;
use axum::extract::{Path, State};
use axum::response::Json;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::query_dsl::InternalJoinDsl;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_types::BigInt;
use diesel::{insert_into, ExpressionMethods, PgConnection, QueryDsl, SelectableHelper};
use futures_util::StreamExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;
use std::time::Duration;

pub(crate) fn analysis_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/analysis_addr/:addr",
      get_with(analysis_addr, |x| { x.description("analysis_addr") }),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    .api_route(
      "/subscribe_addr/:addr",
      get_with(subscribe_addr, |x| { x.description("subscribe_addr") }),
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
pub async fn analysis_addr(Path(addr): Path<String>) -> Result<Json<AnalysisResp>, String> {
  let mainnet_weth = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
  let swaps = match get_user_swaps(addr).await {
    Ok(x) => { x }
    Err(e) => {
      println!("{}", e);
      panic!("error on get grpahql")
    }
  };
  //sell "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2" 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
  let sell1 = swaps.iter().filter(|x| {
    x.pair.token0.id == mainnet_weth && x.amount0_in == "0"
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
    x.pair.token1.id == mainnet_weth && x.amount1_in == "0"
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
    x.pair.token0.id == mainnet_weth && x.amount0_out == "0"
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
    x.pair.token1.id == mainnet_weth && x.amount1_out == "0"
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
    let empty_vec = vec![];
    let sells = sell_map.get(&k).unwrap_or(&empty_vec);
    for x in sells {
      sell_sum += &x.eth_amount
    }
    let mut buy_sum = BigDecimal::from(0);

    for x in &v {
      buy_sum += &x.eth_amount
    }

    if match sells.last() {
      None => { BigDecimal::from(0) }
      Some(x) => { x.token_price.clone() }
    }.gt(&v.first().unwrap().token_price) {
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

sol! {
   interface  IUniswapV2Pair {
    event Swap(
        address indexed sender,
        uint256 amount0In,
        uint256 amount1In,
        uint256 amount0Out,
        uint256 amount1Out,
        address indexed to
    );}
}

pub async fn subscribe_addr(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Path(addr): Path<String>) -> Result<Json<bool>, String> {
  let mut connection = pool.get().unwrap();
  let addr = addr.trim().to_lowercase();
  let count: i64 = addr_subscribes.filter(following_addr.eq(addr.as_str())).count().get_result(&mut connection).unwrap();
  if count >= 1 { return Err("already  subscribed".to_string()); }
  let new_addr_subscribes = NewAddrSubscribes {
    following_addr: addr.clone(),
    subscribers: None,
  };
  let new_addr_subscribes = insert_into(addr_subscribes).values(new_addr_subscribes).returning(AddrSubscribes::as_returning()).get_result(&mut connection).unwrap();
  // tokio::spawn(subscribe(Address::from_hex(addr).unwrap()));
  // ProviderBuilder::new();


  Ok(Json(true))
}

pub async fn listen_and_send(pool: Pool<ConnectionManager<PgConnection>>) {
  tracing::info!("listen_and_send started");
  let mut connection = pool.get().unwrap();

  let rpc_url = env::var("WS_ETH_RPC").unwrap();

  let ws = WsConnect::new(rpc_url);
  let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();

  let filter = Filter::new()
    .event("Swap(address,uint256,uint256,uint256,uint256,address)")
    .from_block(BlockNumberOrTag::Latest);

  // Subscribe to logs.
  let poller = provider.watch_logs(&filter).await.unwrap();
  let mut stream = poller.with_poll_interval(Duration::from_secs(6)).into_stream();

  while let Some(logs) = stream.next().await {
    let subscribes: Vec<AddrSubscribes> = addr_subscribes.filter(deleted.eq(false)).select(AddrSubscribes::as_select()).get_results(&mut connection).unwrap();
    let subscribes: HashSet<_> = subscribes.iter().map(|x| { Address::from_hex(&x.following_addr).unwrap() }).collect();
    for log in logs {
      let swap = log.log_decode::<Swap>().unwrap().inner.data;
      if subscribes.contains(&swap.to) {
        todo!("推送")
      }
    }
  }
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

