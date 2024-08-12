use serde::{Deserialize, Serialize};
use std::error::Error;

mod following_order;
mod tg_user;
mod trading_order;
pub mod param_models;

// pub fn convert<'a, From: Serialize, To:  Deserialize<'a> >(from: &From) -> Result<To, Box<dyn Error>> {
//   let serialised = serde_json::to_string(from)?;
//   let result = serde_json::from_str::<To>(&serialised);
//   match result {
//     Ok(to) => { Ok(to) }
//     Err(e) => { Err(AppError::new_box("as")) }
//   }
// }



