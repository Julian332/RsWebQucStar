#![recursion_limit = "256"]
use aide::axum::ApiRouter;
use aide::axum::routing::{get_with, post_with};
use axum::extract::{Path, State};
use axum::response::Json;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use diesel::r2d2::{ConnectionManager, Pool};

use crate::domain::models::{NewTgUser, TgUser};
use crate::openapi::resp_docs_with_exam;
use crate::schema::tg_user::dsl::tg_user;

pub(crate) fn tg_user_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/create_tg_user",
      post_with(create_tg_user, resp_docs_with_exam::<TgUser>),
      // .get_with(list_todos, empty_resp_docs),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_by_id, resp_docs_with_exam::<TgUser>),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    // .api_route("/:id", put_with(complete_todo, empty_resp_docs))
    .with_state(conn_pool)
}

async fn create_tg_user(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(mut new_user): Json<NewTgUser>) -> Result<Json<TgUser>, String> {
  let mut connection = pool.get().unwrap();
  // let string = new_user.address.to_lowercase();
  new_user.address = new_user.address.trim().to_lowercase();
  let result = diesel::insert_into(tg_user).values(new_user).returning(TgUser::as_returning()).get_result(&mut connection).expect("Error saving new TgUser");

  Ok(Json::from(result))
}
async fn get_by_id(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i64>) -> Result<Json<Option<TgUser>>, String> {
  let mut connection = pool.get().unwrap();
  let result = tg_user.find(id_param).select(TgUser::as_select()).first(&mut connection).optional().unwrap();
  Ok(Json(result))
}

