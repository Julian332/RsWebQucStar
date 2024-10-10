use crate::api_auth::login_strategy::AuthBackend;
use crate::controller::{PageParam, PageRes};
use crate::models::User;
use crate::openapi::{default_resp_docs_with_exam, empty_resp_docs};
use crate::schema::users::dsl::users;
use aide::axum::routing::{delete_with, get_with, post_with, put_with};
use aide::axum::ApiRouter;
use axum::extract::{Path, State};
use axum::response::Json;
use axum_login::login_required;
use chrono::{DateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub group_id: i64,
    pub tenantry: String,
    pub remark: Option<String>,
    pub update_time: Option<DateTime<Utc>>,
    pub create_time: DateTime<Utc>,
    pub create_by: i64,
    pub update_by: Option<i64>,
    pub is_delete: bool,
}

pub(crate) fn web_routes(conn_pool: Pool<ConnectionManager<PgConnection>>) -> ApiRouter {
    ApiRouter::new()
        .api_route("/create_entity", post_with(create_entity, empty_resp_docs))
        .api_route(
            "/get_entity_by_id/:id",
            get_with(get_entity_by_id, default_resp_docs_with_exam::<User>),
        )
        .api_route(
            "/update_entity_by_id/:id",
            put_with(update_entity_by_id, default_resp_docs_with_exam::<User>),
        )
        .api_route(
            "/delete_entity_by_id/:id",
            delete_with(delete_entity_by_id, default_resp_docs_with_exam::<User>),
        )
        .api_route(
            "/get_entity_page",
            post_with(
                get_entity_page,
                default_resp_docs_with_exam::<PageRes<User>>,
            ),
        )
        .with_state(conn_pool)
        .route_layer(login_required!(AuthBackend, login_url = "/login"))
}
async fn create_entity(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(new_entity): Json<NewUser>,
) -> Result<Json<User>, String> {
    let mut connection = pool.get().unwrap();

    let result = diesel::insert_into(users)
        .values(new_entity)
        .returning(User::as_returning())
        .get_result(&mut connection)
        .expect("Error saving new entity");

    Ok(Json::from(result))
}
async fn update_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
    Json(new): Json<NewUser>,
) -> Result<Json<User>, String> {
    let mut connection = pool.get().unwrap();
    let result = diesel::update(users.find(id_param))
        .set(&new)
        .returning(User::as_returning())
        .get_result(&mut connection)
        .expect("Error update  entity");
    Ok(Json(result))
}
async fn get_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
) -> Result<Json<User>, String> {
    let mut connection = pool.get().unwrap();
    let result = users
        .find(id_param)
        .select(User::as_select())
        .get_result(&mut connection)
        .expect("get entity by id failed");
    Ok(Json(result))
}
async fn delete_entity_by_id(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(id_param): Path<i64>,
) -> Result<Json<User>, String> {
    let mut connection = pool.get().unwrap();
    let result = diesel::update(users.find(id_param))
        .set(crate::schema::users::is_delete.eq(true))
        .returning(User::as_returning())
        .get_result(&mut connection)
        .expect("Error delete  entity");
    Ok(Json(result))
}
async fn get_entity_page(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(page): Json<PageParam<User>>,
) -> Result<Json<PageRes<User>>, String> {
    let mut connection = pool.get().unwrap();
    let off_lim = page.get_offset_limit();
    let res = users
        .limit(off_lim.1)
        .offset(off_lim.0)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading page");
    let page_res = PageRes::from_param_records(page, res);
    Ok(Json(page_res))
}
