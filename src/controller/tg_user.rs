use crate::models::{NewTgUser, TgUser};
use crate::schema::tg_user::dsl::tg_user;
use crate::schema::tg_user::address;
use crate::{web_fn_gen, web_router_gen};


web_router_gen! {tg_user,NewTgUser,TgUser}

pub fn user_by_addr(addr: Address, connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> Option<TgUser> {
  tg_user.filter(address.eq(addr.to_string().to_lowercase())).select(TgUser::as_select()).first(connection).optional().unwrap()
}


