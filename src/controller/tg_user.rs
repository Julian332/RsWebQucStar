use aide::axum::{ApiRouter, IntoApiResponse};
use aide::axum::routing::{get_with, post_with, put_with};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use axum_macros::debug_handler;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper, Table};
use diesel::r2d2::{ConnectionManager, Pool};
use crate::domain::models::{NewTgUser, TgUser};
use crate::openapi::{empty_resp_docs, resp_docs_with_exam};
use crate::schema::tg_user::dsl::tg_user;
use crate::schema::tg_user::id;

pub(crate) fn tg_user_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/create_tg_user",
      post_with(create_tg_user, resp_docs_with_exam::<TgUser>),
      // .get_with(list_todos, empty_resp_docs),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_tg_user, resp_docs_with_exam::<TgUser>),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    // .api_route("/:id", put_with(complete_todo, empty_resp_docs))
    .with_state(conn_pool)
}

async fn create_tg_user(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(new_user): Json<NewTgUser>) -> Result<Json<TgUser>, String> {
  let mut connection = pool.get().unwrap();

  let result = diesel::insert_into(tg_user).values(new_user).returning(TgUser::as_returning()).get_result(&mut connection).expect("Error saving new TgUser");

  Ok(Json::from(result))
}
#[debug_handler]
async fn get_tg_user(
  State(pool): State<Pool<ConnectionManager<PgConnection>>>,
  Path(id_param): Path<i32>) -> Result<Json<TgUser>, String> {
  let mut connection = pool.get().unwrap();
  // tg_user.filter(id.eq(id_param)).select(TgUser::as_select()).load(&mut connection).expect("Error loading posts");
  let result = tg_user.find(id_param).select(TgUser::as_select()).first(&mut connection).optional();

  return;
}


