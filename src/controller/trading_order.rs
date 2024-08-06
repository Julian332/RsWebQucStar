use std::env;
use std::error::Error;
use std::str::FromStr;

use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with, put_with};
use aide::transform::TransformOperation;
use alloy::hex::FromHex;
use alloy::primitives::{Address, TxHash, U256};
use axum::extract::{Path, State};
use axum::response::Json;
use bigdecimal::ToPrimitive;
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
    "trading" => { println!("{:?}", new_order.make_trading(&mut connection).await); }
    "pending" => {}
    "following" => {}
    _ => { unreachable!() }
  }


  let result = diesel::insert_into(trading_order).values(new_order).returning(TradingOrder::as_returning()).get_result(&mut connection).expect("Error saving new TradingOrder");
  Ok(Json::from(result))
}


impl NewTradingOrder {
  async fn make_trading(&self, conn: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<TxHash, Box<dyn Error>> {
    let user_address = Address::from_hex(self.user_addr.clone())?;
    let user = user_by_addr(user_address, conn).unwrap();
    let chain_id = isize::from_str(env::var("CHAIN_ID")?.as_str())?;

    let uni_router2 = get_uni_router2().await?;
    match self.sell_or_buy.clone().as_str() {
      "sell" => {
        let amount_in = self.from_token_amount.clone().unwrap();
        let path: Vec<Address> = [env::var("WETH_ROUTER2_ADDR")?, self.target_token.clone()]
          .iter().map(|x| { Address::from_hex(x).unwrap() }).collect();
        let amounts_out_return = uni_router2.getAmountsOut(U256::from(amount_in.to_u128().unwrap()), path.clone()).call().await?;
        let target_amount = amounts_out_return.amounts.get(1).unwrap();
        let deadline = chrono::Utc::now().timestamp()+180;

        //todo slippage
        let swap_res = uni_router2.swapExactETHForTokensSupportingFeeOnTransferTokens(*target_amount,path,user_address,U256::from(deadline)).send().await?;
        //
        // let handle = tokio::spawn(move || {});
        // return Ok(swap_res.tx_hash().clone());
      }
      "buy" => {}
      _ => { unreachable!() }
    }


    //todo
    let from_amount = self.from_token_amount.clone().unwrap();
    // let aggr = dex_aggr(chain_id, from_amount, Address::from_str(self.target_token.as_str())?, Address::from_str(self.from_token.as_str())?, self.slippage.clone(), user_address).await?;
    // println!("{}", aggr.code);
    //todo
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
        target_token: "".to_string(),
        from_token: "".to_string(),
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