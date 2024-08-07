use std::env;
use std::error::Error;
use std::ops::{Mul, Sub};

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use aide::transform::TransformOperation;
use alloy::hex;
use alloy::hex::FromHex;
use alloy::network::EthereumWallet;
use alloy::primitives::{Address, TxHash, U256};
use alloy::signers::local::PrivateKeySigner;
use axum::extract::{Path, State};
use axum::response::Json;
use bigdecimal::{BigDecimal, One, ToPrimitive};
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use crate::contract::uni_router2::get_uni_router2;
use crate::controller::{PageParam, PageRes};
use crate::controller::tg_user::user_by_addr;
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

async fn create_trading_order(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(mut new_order): Json<NewTradingOrder>) -> Result<Json<TradingOrder>, String> {
  let mut connection = pool.get().unwrap();
  new_order.target_token = new_order.target_token.trim().to_lowercase();
  new_order.from_token = new_order.from_token.trim().to_lowercase();
  new_order.order_type = new_order.order_type.trim().to_lowercase();
  match new_order.order_type.as_str() {
    "trading" => {
      let result1 = new_order.make_trading(&mut connection).await;
      println!("{:?}", result1); }
    "pending" => {}
    "following" => {}
    _ => { unreachable!() }
  }
  //todo tx fee


  let result = diesel::insert_into(trading_order).values(new_order).returning(TradingOrder::as_returning()).get_result(&mut connection).expect("Error saving new TradingOrder");
  Ok(Json::from(result))
}


impl NewTradingOrder {
  async fn make_trading(&self, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<TxHash, Box<dyn Error>> {
    let user_address = Address::from_hex(self.user_addr.clone())?;
    let user = user_by_addr(user_address, conn).unwrap();
    let pk = hex::decode(user.private_key.unwrap())?;
    // let chain_id = isize::from_str(env::var("CHAIN_ID")?.as_str())?;
    let wallet = EthereumWallet::from(PrivateKeySigner::from_slice(&pk)?);
    let uni_router2 = get_uni_router2(wallet).await?;

    match self.sell_or_buy.clone().as_str() {
      "buy" => {
        
      }
      "sell" => {

        // return Ok(swap_res.tx_hash().clone());
      }
      _ => { unreachable!() }
    }
    
    
    match (&self.from_token_amount, &self.target_amount) {
      (Some(amount_in), None) => {

        // if self.from_token.eq_ignore_ascii_case("eth") { env::var("WETH_ROUTER2_ADDR")? } else { self.from_token.clone() }

        // let amount_in = self.from_token_amount.clone().unwrap();
        let mut tx_value = 0;
        let path: Vec<Address> = [if self.from_token.eq_ignore_ascii_case("eth") {
          tx_value = amount_in.to_u128().unwrap();
          env::var("WETH_ROUTER2_ADDR")?
        } else { self.from_token.clone() }
          , self.target_token.clone()]
          .iter().map(|x| { Address::from_hex(x).unwrap() }).collect();

        let amounts_out_return = uni_router2.getAmountsOut(U256::from(amount_in.to_u128().unwrap()), path.clone()).call().await?;
        let mut target_amount = *amounts_out_return.amounts.get(1).unwrap();
        if let Some(slippage) = self.slippage.clone() {
          let decimal = BigDecimal::from(target_amount.to::<u128>()).mul(BigDecimal::one().sub(slippage));
          target_amount = U256::from(decimal.to_u128().unwrap());
        }
        let deadline = chrono::Utc::now().timestamp() + 180;

        let builder1;
        let builder2;
        let swap_res =
          if self.from_token.eq_ignore_ascii_case("eth") {
            builder1 = uni_router2.swapExactETHForTokensSupportingFeeOnTransferTokens(
              target_amount,
              path,
              user_address,
              U256::from(deadline))
              .value(U256::from(tx_value));
            builder1
              .send().await?
          } else {
            builder2 = uni_router2.swapExactTokensForTokens(
              U256::from(amount_in.to_u128().unwrap()),
              target_amount,
              path,
              user_address,
              U256::from(deadline));
            builder2
              .send().await?
          };
        return Ok(*swap_res.tx_hash());
        //
        // let handle = tokio::spawn(move || {});
      }
      (None, Some(amount_out)) => {

        // if self.from_token.eq_ignore_ascii_case("eth") { env::var("WETH_ROUTER2_ADDR")? } else { self.from_token.clone() }

        // let amount_in = self.from_token_amount.clone().unwrap();
        let mut tx_value = 0;
        let path: Vec<Address> = [if self.from_token.eq_ignore_ascii_case("eth") {
          env::var("WETH_ROUTER2_ADDR")?
        } else { self.from_token.clone() }
          , self.target_token.clone()]
          .iter().map(|x| { Address::from_hex(x).unwrap() }).collect();
        
        let amounts_out_return = uni_router2.getAmountsIn(U256::from(amount_out.to_u128().unwrap()), path.clone()).call().await?;
        let mut target_amount = *amounts_out_return.amounts.get(1).unwrap();
        
        if let Some(slippage) = self.slippage.clone() {
          let decimal = BigDecimal::from(target_amount.to::<u128>()).mul(BigDecimal::one().sub(slippage));
          target_amount = U256::from(decimal.to_u128().unwrap());
        }
        let deadline = chrono::Utc::now().timestamp() + 180;

        let builder1;
        let builder2;
        let swap_res =
          if self.from_token.eq_ignore_ascii_case("eth") {
            builder1 = uni_router2.swapETHForExactTokens(
              target_amount,
              path,
              user_address,
              U256::from(deadline))
              .value(U256::from(tx_value));
            builder1
              .send().await?
          } else {
            builder2 = uni_router2.swapExactTokensForTokens(
              U256::from(amount_in.to_u128().unwrap()),
              target_amount,
              path,
              user_address,
              U256::from(deadline));
            builder2
              .send().await?
          };
        return Ok(*swap_res.tx_hash());
        //
        // let handle = tokio::spawn(move || {});
      }
      _ => { panic!("wrong amount_in amount_out") }
    }



    return Err(AppError::new_box("un"));
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
    .response_with::<200, crate::openapi::extractors::Json<NewTradingOrder>, _>(|res| {
      res.example(NewTradingOrder {
        sell_or_buy: "sell|buy".to_string(),
        target_token: "erc20 address ".to_string(),
        from_token: "erc20 address | eth".to_string(),
        trading_uer: 0,
        boost_mode: false,
        mev_protected: false,
        priority_fee: None,
        target_amount: None,
        from_token_amount: None,
        pending_target_price: None,
        expire_at: None,
        order_type: "trading|pending|following".to_string(),
        slippage: None,
        user_addr: "".to_string(),
      })
    })
}