use crate::models::{NewTgUser, TgUser};
use axum::extract::State;
use axum::Json;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Identifiable, Insertable, PgConnection, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod tg_user;
pub mod trading_order;
pub mod analysis;
pub mod macros;

#[derive(
  Debug,
  Serialize,
  Deserialize,
  Default,
  JsonSchema,
)]
pub struct PageParam<T> {
  model: T,
  page_no: i64,
  page_size: i64,
}

#[derive(
  Debug,
  Serialize,
  Deserialize,
  Default,
  JsonSchema,
)]
pub struct PageRes<T> {
  page_no: i64,
  page_size: i64,
  records: Vec<T>,
  total_count: i64,
}

impl<T> PageRes<T> {
  pub fn from_param_records(param: PageParam<T>, records: Vec<T>) -> PageRes<T> {
    PageRes {
      page_no: param.page_no,
      page_size: param.page_size,
      records,
      total_count: -1,
    }
  }
  pub fn from_param_records_count(param: PageParam<T>, records: Vec<T>, total_count: i64) -> PageRes<T> {
    PageRes {
      page_no: param.page_no,
      page_size: param.page_size,
      records,
      total_count,
    }
  }
}

impl<T> PageParam<T> {
  fn get_offset_limit(&self) -> (i64, i64) {
    ((self.page_no - 1) * self.page_size, self.page_size)
  }
}

#[macro_export]
macro_rules! web_fn_gen {
  ($table:ident ,$new:ident, $result:ident) => {
    async fn create_entity (State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(new_entity): Json<$new>) -> Result<Json<$result>, String> {
      let mut connection = pool.get().unwrap();

      let result = diesel::insert_into($table).values(new_entity).returning($result::as_returning()).get_result(&mut connection).expect("Error saving new entity");

      Ok(Json::from(result))
    }

    async fn update_entity_by_id(
      State(pool): State<Pool<ConnectionManager<PgConnection>>>,
      Path(id_param): Path<i64>,
      Json(new): Json<$new>) -> Result<Json<$result>, String> {
      let mut connection = pool.get().unwrap();
      let result = diesel::update(tg_user.find(id_param)).set(&new).returning($result::as_returning()).get_result(&mut connection).expect("Error update  entity");
      Ok(Json(result))
    }

    async fn get_entity_by_id(
      State(pool): State<Pool<ConnectionManager<PgConnection>>>,
      Path(id_param): Path<i64>) -> Result<Json<$result>, String> {
      let mut connection = pool.get().unwrap();
      let result = tg_user.find(id_param).select($result::as_select()).get_result(&mut connection).expect("get entity by id failed");
      Ok(Json(result))
    }

    async fn delete_entity_by_id(
      State(pool): State<Pool<ConnectionManager<PgConnection>>>,
      Path(id_param): Path<i64>) -> Result<Json<$result>, String> {
      let mut connection = pool.get().unwrap();
      let result = diesel::update(tg_user.find(id_param)).set(crate::schema::$table::deleted.eq(true)).returning($result::as_returning()).get_result(&mut connection).expect("Error delete  entity");
      Ok(Json(result))
    }

    async fn get_entity_page(
      State(pool): State<Pool<ConnectionManager<PgConnection>>>,
      Json(page): Json<PageParam<$result>>) -> Result<Json<PageRes<$result>>, String> {
      let mut connection = pool.get().unwrap();
      let off_lim = page.get_offset_limit();
      let res = $table.limit(off_lim.1).offset(off_lim.0).select($result::as_select()).load(&mut connection).expect("Error loading page");
      let page_res = PageRes::from_param_records(page, res);
      Ok(Json(page_res))
    }
  };
}


