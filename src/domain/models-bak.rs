// use diesel::prelude::*;
// 
// #[derive(Queryable, Selectable,Insertable)]
// #[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }
// 
// use crate::schema::posts;
// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
// 
// 
// #[derive(Queryable, Selectable,Insertable)]
// #[diesel(table_name = crate::schema::tg_user)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct TgUser {
//     pub id: u32, // id
//     pub deleted: bool, // deleted
//     pub create_time: String, // create_time
//     pub update_time: String, // update_time
//     pub address: String, // address
//     pub private_key: String, // private_key
//     pub fee_staged: u32, // fee_staged
//     pub fee_received: u32, // fee_recrived
//     pub parent: String, // parent
// }