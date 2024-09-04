use aide::axum::routing::{get_with, post_with, put_with};
use aide::axum::ApiRouter;
use alloy::primitives::Address;
use axum::extract::{Path, State};
use axum::response::Json;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::controller::{PageParam, PageRes};
use crate::models::{NewTgUser, TgUser};
use crate::openapi::{default_resp_docs_with_exam, empty_resp_docs};
use crate::schema::tg_user::dsl::tg_user;
use crate::schema::tg_user::address;
use crate::web_fn_gen;


pub(crate) fn tg_user_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
  ApiRouter::new()
    .api_route(
      "/create_tg_user",
      post_with(create_entity, empty_resp_docs),
      // .get_with(list_todos, empty_resp_docs),
    )
    .api_route(
      "/get_by_id/:id",
      get_with(get_entity_by_id, default_resp_docs_with_exam::<TgUser>),
      // .delete_with(delete_todo, empty_resp_docs),
    )
    .api_route("/update_by_id/:id", put_with(update_entity_by_id, default_resp_docs_with_exam::<TgUser>))
    .api_route("/page", post_with(get_entity_page, default_resp_docs_with_exam::<PageRes<TgUser>>))
    .with_state(conn_pool)
}


pub fn user_by_addr(addr: Address, connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Option<TgUser> {
  tg_user.filter(address.eq(addr.to_string().to_lowercase())).select(TgUser::as_select()).first(connection).optional().unwrap()
}

web_fn_gen! {tg_user,NewTgUser,TgUser}
