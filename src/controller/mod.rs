use diesel::{Identifiable, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod tg_user;
pub mod trading_order;
pub mod analysis;

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