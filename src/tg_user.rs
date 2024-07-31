use diesel::{Queryable, Selectable};

// tg_user
// Created by julian on 2024-07-31.
// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::posts)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Default, Debug)]
pub struct TgUser {
    pub id: String, // id
    pub deleted: String, // deleted
    pub create_time: String, // create_time
    pub update_time: String, // update_time
    pub address: String, // address
    pub private_key: String, // private_key
    pub fee_staged: u32, // fee_staged
    pub fee_received: u32, // fee_recrived
    pub parent: String, // parent
}