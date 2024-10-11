use crate::controller::{PageParam, PageRes, LOGIN_URL};
use crate::models::Auction;
use crate::openapi::{default_resp_docs_with_exam, empty_resp_docs};
use crate::schema::auction::dsl::auction;

use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use axum::extract::{Path, State};
use axum::response::Json;
use axum_login::login_required;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel::{
    ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::api_auth::login_impl::AuthBackend;

#[derive(
    Queryable,
    Clone,
    Serialize,
    Deserialize,
    Selectable,
    JsonSchema,
    Default,
    Insertable,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::auction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAuction {
    pub token_addr: String,
    pub name: String,
    pub symbol: String,
    pub once_amount: i64,
    pub total_supply: BigDecimal,
    pub total_eth: BigDecimal,
    pub start_time: DateTime<Utc>,
    pub publish_time: DateTime<Utc>,
    pub is_burn_lp_token: bool,
    pub creator_addr: String,
    pub creator_id: String,
    pub transaction_hash: String,
    pub description: String,
    pub image: String,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub is_delete: bool,
    pub is_published: bool,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct NewAuctionParams {
    pub token_addr: String,
    pub name: String,
    pub symbol: String,
    pub once_amount: i64,
    pub total_supply: BigDecimal,
    pub total_eth: BigDecimal,
    pub start_time: DateTime<Utc>,
    pub publish_time: DateTime<Utc>,
    pub is_burn_lp_token: bool,
    pub creator_addr: String,
    pub transaction_hash: String,
    pub description: String,
    pub image: String,
}

pub(crate) fn web_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    ApiRouter::new()
        .api_route("/create_entity", post_with(create_entity, empty_resp_docs))
        .api_route(
            "/get_entity_by_id/:id",
            get_with(get_entity_by_id, default_resp_docs_with_exam::<Auction>),
        )
        .api_route(
            "/update_entity_by_id/:id",
            put_with(update_entity_by_id, default_resp_docs_with_exam::<Auction>),
        )
        .api_route(
            "/delete_entity_by_id/:id",
            delete_with(delete_entity_by_id, default_resp_docs_with_exam::<Auction>),
        )
        .api_route(
            "/get_entity_page",
            post_with(
                get_entity_page,
                default_resp_docs_with_exam::<PageRes<Auction>>,
            ),
        )
        .with_state(conn_pool)
        .route_layer(login_required!(AuthBackend, login_url = LOGIN_URL))
}
async fn create_entity(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(new_entity): Json<NewAuction>,
) -> Result<Json<Auction>, String> {
    let mut connection = pool.get().unwrap();

    let result = diesel::insert_into(auction)
        .values(new_entity)
        .returning(Auction::as_returning())
        .get_result(&mut connection)
        .expect("Error saving new entity");

    Ok(Json::from(result))
}
async fn update_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
    Json(new): Json<NewAuction>,
) -> Result<Json<Auction>, String> {
    let mut connection = pool.get().unwrap();
    let result = diesel::update(auction.find(id_param))
        .set(&new)
        .returning(Auction::as_returning())
        .get_result(&mut connection)
        .expect("Error update  entity");
    Ok(Json(result))
}
async fn get_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
) -> Result<Json<Auction>, String> {
    let mut connection = pool.get().unwrap();
    let result = auction
        .find(id_param)
        .select(Auction::as_select())
        .get_result(&mut connection)
        .expect("get entity by id failed");
    Ok(Json(result))
}
async fn delete_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
) -> Result<Json<Auction>, String> {
    let mut connection = pool.get().unwrap();
    let result = diesel::update(auction.find(id_param))
        .set(crate::schema::auction::is_delete.eq(true))
        .returning(Auction::as_returning())
        .get_result(&mut connection)
        .expect("Error delete  entity");
    Ok(Json(result))
}
async fn get_entity_page(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(page): Json<PageParam<Auction>>,
) -> Result<Json<PageRes<Auction>>, String> {
    let mut connection = pool.get().unwrap();
    let off_lim = page.get_offset_limit();
    let res = auction
        .limit(off_lim.1)
        .offset(off_lim.0)
        .select(Auction::as_select())
        .load(&mut connection)
        .expect("Error loading page");
    let page_res = PageRes::from_param_records(page, res);
    Ok(Json(page_res))
}
