use std::env;
use std::error::Error;
use std::ops::{Div, Mul, Sub};
use std::str::FromStr;

use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use aide::transform::TransformOperation;
use alloy::hex;
use alloy::hex::FromHex;
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::{Address, TxHash, Uint, U256};
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::providers::{Provider, ProviderBuilder, ReqwestProvider};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use alloy::transports::http::{Client, Http};
use axum::extract::{Path, State};
use axum::response::Json;
use bigdecimal::{BigDecimal, One, ToPrimitive};
use diesel::query_dsl::InternalJoinDsl;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::contract::uni_router2::get_uni_router2;
use crate::contract::uni_router2::UNI_ROUTER2::UNI_ROUTER2Instance;
use crate::contract::{transfer, transfer_with_nonce};
use crate::controller::tg_user::user_by_addr;
use crate::controller::{PageParam, PageRes};
use crate::domain::models::{NewTradingOrder, TradingOrder};
use crate::openapi::default_resp_docs_with_exam;
use crate::openapi::errors::AppError;
use crate::schema::trading_order::dsl::trading_order;
use crate::schema::trading_order::table;

pub(crate) fn trading_order_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/create_trading_order",
      post_with(create_trading_order, create_trading_order_doc),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_by_id, default_resp_docs_with_exam::<TradingOrder>),
    )
    .api_route("/update_by_id/:id", put_with(update_by_id, default_resp_docs_with_exam::<TradingOrder>))
    .api_route("/page", post_with(page, default_resp_docs_with_exam::<PageRes<TradingOrder>>))
    .with_state(conn_pool)
}

async fn create_trading_order(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(mut new_order): Json<NewTradingOrder>) -> Result<Json<String>, String> {
  let mut connection = pool.get().unwrap();
  new_order.target_token = new_order.target_token.trim().to_lowercase();
  new_order.from_token = new_order.from_token.trim().to_lowercase();
  new_order.order_type = new_order.order_type.trim().to_lowercase();
  match new_order.order_type.as_str() {
    "trading" => {}
    "pending" => {}
    "following" => {}
    _ => { unreachable!() }
  }

  match new_order.make_trading(&mut connection).await {
    Ok((tx_hash, eth)) => {
      let result = diesel::insert_into(trading_order).values(new_order).returning(TradingOrder::as_returning()).get_result(&mut connection).expect("Error saving new TradingOrder");


      Ok(Json::from(tx_hash.to_string()))
    }
    Err(e) => {
      println!("{}", e);
      tracing::error!("error: {}",e);
      Err("internal error".to_string())
    }
  }
}


impl NewTradingOrder {
  async fn make_trading(&self, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<(TxHash, U256), Box<dyn Error>> {
    let user_address = Address::from_hex(self.user_addr.clone())?;
    let user = user_by_addr(user_address, conn).unwrap();
    let pk = hex::decode(user.private_key.unwrap())?;
    let wallet = EthereumWallet::from(PrivateKeySigner::from_slice(&pk)?);
    let provider = ProviderBuilder::new()
      .with_recommended_fillers()
      .wallet(wallet)
      .on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());
    let uni_router2 = get_uni_router2(provider.clone()).await?;
    let count = provider.get_transaction_count(user_address).await?;

    match self.sell_or_buy.clone().to_lowercase().as_str() {
      "buy" => {
        let amount_in = U256::from(self.from_token_amount.clone().to_u128().unwrap());
        let path: Vec<Address> = [env::var("WETH_ROUTER2_ADDR")?, self.target_token.clone()]
          .iter().map(|x| { Address::from_hex(x).unwrap() }).collect();

        let target_amount = self.get_amount_out(&uni_router2, amount_in, &path).await?;
        let deadline = chrono::Utc::now().timestamp() + 180;

        //todo  gas fee
        let builder = uni_router2.swapExactETHForTokensSupportingFeeOnTransferTokens(
          target_amount,
          path,
          user_address,
          U256::from(deadline))
          .value(U256::from(amount_in));
        let swap_res = builder
          .send().await?;

        let fee_tx_hash = transfer_with_nonce(
          provider.clone(),
          Address::from_str(env::var("PROJECT_SIDE_ADDR")?.as_str()).unwrap(),
          amount_in.div(U256::from(100)),
          count + 1).await?;
        return Ok((*swap_res.tx_hash(), amount_in));
      }
      "sell" => {
        let amount_in = U256::from(self.from_token_amount.clone().to_u128().unwrap());
        let path: Vec<Address> = [self.from_token.clone(), env::var("WETH_ROUTER2_ADDR")?]
          .iter().map(|x| { Address::from_hex(x).unwrap() }).collect();

        let target_amount = self.get_amount_out(&uni_router2, amount_in, &path).await?;

        let deadline = chrono::Utc::now().timestamp() + 180;

        let call_builder = uni_router2.swapExactTokensForETHSupportingFeeOnTransferTokens(
          amount_in,
          target_amount,
          path,
          user_address,
          U256::from(deadline));

        let swap_res = call_builder
          .send().await?;

        let fee_tx_hash = transfer_with_nonce(
          provider.clone(),
          Address::from_str(env::var("PROJECT_SIDE_ADDR")?.as_str()).unwrap(),
          target_amount.div(U256::from(100)),
          count + 1).await?;

        return Ok((*swap_res.tx_hash(), target_amount));
      }
      _ => { panic!("only sell or buy") }
    }
  }

  async fn get_amount_out(&self, uni_router2: &UNI_ROUTER2Instance<Http<Client>, FillProvider<JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum>>, amount_in: Uint<256, 4>, path: &Vec<Address>) -> Result<U256, Box<dyn Error>> {
    let amounts_out_return = uni_router2.getAmountsOut(amount_in, path.clone()).call().await?;
    let mut target_amount = *amounts_out_return.amounts.get(1).unwrap();
    if let Some(slippage) = self.slippage.clone() {
      let decimal = BigDecimal::from(target_amount.to::<u128>()).mul(BigDecimal::one().sub(slippage));
      target_amount = U256::from(decimal.to_u128().unwrap());
    }
    Ok(target_amount)
  }
}
async fn get_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>) -> Result<Json<Option<TradingOrder>>, String> {
  let mut connection = pool.get().unwrap();
  let table = trading_order;
  let result = table.find(id_param).first(&mut connection).optional().unwrap();
  Ok(Json(result))
}


async fn update_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>,
  Json(order): Json<TradingOrder>) -> Result<Json<bool>, String> {
  let mut connection = pool.get().unwrap();
  let result = diesel::update(trading_order.find(id_param)).set(&order).execute(&mut connection).unwrap();

  Ok(Json(result == 1))
}

async fn page(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Json(page): Json<PageParam<TradingOrder>>) -> Result<Json<PageRes<TradingOrder>>, String> {
  let mut connection = pool.get().unwrap();
  let off_lim = page.get_offset_limit();
  let res = trading_order.limit(off_lim.1).offset(off_lim.0).select(TradingOrder::as_select()).load(&mut connection).expect("Error loading page");
  let page_res = PageRes::from_param_records(page, res);
  Ok(Json(page_res))
}

pub fn create_trading_order_doc(op: TransformOperation) -> TransformOperation {
  op.description("default_docs")
    // .parameter::< crate::openapi::extractors::Json<NewTradingOrder>, _>("a",|res| {
    //   res.example(NewTradingOrder {
    //     sell_or_buy: "sell|buy".to_string(),
    //     target_token: "erc20 address ".to_string(),
    //     from_token: "erc20 address | eth".to_string(),
    //     // trading_uer: 0,
    //     boost_mode: false,
    //     mev_protected: false,
    //     priority_fee: None,
    //     // target_amount: None,
    //     from_token_amount: BigDecimal::from(100000),
    //     // pending_target_price: None,
    //     // expire_at: None,
    //     order_type: "trading|pending|following".to_string(),
    //     slippage: None,
    //     user_addr: "".to_string(),
    //   })
    // })
    .response_with::<200, crate::openapi::extractors::Json<NewTradingOrder>, _>(|res| {
    res.example(NewTradingOrder {
      sell_or_buy: "sell|buy".to_string(),
      target_token: "erc20 address ".to_string(),
      from_token: "erc20 address | eth".to_string(),
      // trading_uer: 0,
      boost_mode: false,
      mev_protected: false,
      priority_fee: None,
      // target_amount: None,
      from_token_amount: BigDecimal::from(100000),
      // pending_target_price: None,
      // expire_at: None,
      order_type: "trading|pending|following".to_string(),
      slippage: None,
      user_addr: "".to_string(),
    })
  })
}