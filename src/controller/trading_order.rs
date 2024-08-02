use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use axum::extract::{Path, State};
use axum::response::Json;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::domain::models::TradingOrder;
use crate::openapi::resp_docs_with_exam;
use crate::schema::trading_order::dsl::trading_order;

pub(crate) fn trading_order_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/trading_order",
      post_with(create_trading_order, resp_docs_with_exam::<TradingOrder>),
      // .get_with(list_todos, empty_resp_docs),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_by_id, resp_docs_with_exam::<TradingOrder>),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    // .api_route("/:id", put_with(complete_todo, empty_resp_docs))
    .with_state(conn_pool)
}

async fn create_trading_order(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(mut new_order): Json<TradingOrder>) -> Result<Json<TradingOrder>, String> {
  let mut connection = pool.get().unwrap();
  new_order.target_token = new_order.target_token.trim().to_lowercase();
  new_order.from_token = new_order.from_token.trim().to_lowercase();
  let result = diesel::insert_into(trading_order).values(new_order).returning(TradingOrder::as_returning()).get_result(&mut connection).expect("Error saving new TradingOrder");
  Ok(Json::from(result))
}


async fn get_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>) -> Result<Json<Option<TradingOrder>>, String> {
  let mut connection = pool.get().unwrap();
  let result = trading_order.find(id_param).first(&mut connection).optional().unwrap();
  Ok(Json(result))
}

