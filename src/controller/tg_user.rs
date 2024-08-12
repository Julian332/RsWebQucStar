use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use alloy::primitives::Address;
use alloy::signers::k256::elliptic_curve::generic_array::typenum::private::Trim;
use axum::extract::{Path, State};
use axum::response::Json;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::controller::{PageParam, PageRes};
use crate::models::{NewTgUser, TgUser};
use crate::openapi::default_resp_docs_with_exam;
use crate::schema::tg_user::address;
use crate::schema::tg_user::dsl::tg_user;

pub(crate) fn tg_user_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/create_tg_user",
      post_with(create_tg_user, default_resp_docs_with_exam::<TgUser>),
      // .get_with(list_todos, empty_resp_docs),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_user_by_id, default_resp_docs_with_exam::<TgUser>),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    .api_route("/update_by_id/:id", put_with(update_by_id, default_resp_docs_with_exam::<TgUser>))
    .api_route("/page", post_with(page, default_resp_docs_with_exam::<PageRes<TgUser>>))
    .with_state(conn_pool)
}

async fn create_tg_user(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(mut new_user): Json<NewTgUser>) -> Result<Json<TgUser>, String> {
  let mut connection = pool.get().unwrap();
  // let string = new_user.address.to_lowercase();
  new_user.address = new_user.address.trim().to_lowercase();
  // if let Some(private_key) = new_user.private_key {
  //   new_user.private_key = Some(private_key.trim());
  // }
  
  let result = diesel::insert_into(tg_user).values(new_user).returning(TgUser::as_returning()).get_result(&mut connection).expect("Error saving new TgUser");

  Ok(Json::from(result))
}
pub async fn get_user_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>) -> Result<Json<Option<TgUser>>, String> {
  let mut connection = pool.get().unwrap();
  let result = tg_user.find(id_param).select(TgUser::as_select()).first(&mut connection).optional().unwrap();
  Ok(Json(result))
}

pub async fn get_user_by_addr(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(addr): Path<Address>) -> Result<Json<Option<TgUser>>, String> {
  let mut connection = pool.get().unwrap();
  let result = user_by_addr(addr, &mut connection);
  // let result = tg_user.find(id_param).select(TgUser::as_select()).first(&mut connection).optional().unwrap();
  Ok(Json(result))
}

pub fn user_by_addr(addr: Address, connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Option<TgUser> {
  tg_user.filter(address.eq(addr.to_string().to_lowercase())).select(TgUser::as_select()).first(connection).optional().unwrap()
}

async fn update_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>,
  Json(user): Json<TgUser>) -> Result<Json<bool>, String> {
  let mut connection = pool.get().unwrap();
  let result = diesel::update(tg_user.find(id_param)).set(&user).execute(&mut connection).unwrap();
  Ok(Json(result == 1))
}

async fn page(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Json(page): Json<PageParam<TgUser>>) -> Result<Json<PageRes<TgUser>>, String> {
  let mut connection = pool.get().unwrap();
  let off_lim = page.get_offset_limit();
  let res = tg_user.limit(off_lim.1).offset(off_lim.0).select(TgUser::as_select()).load(&mut connection).expect("Error loading page");
  let page_res = PageRes::from_param_records(page, res);
  Ok(Json(page_res))
}
