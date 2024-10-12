use crate::contract::uni_factory::{uni_factory_addr, UNI_FACTORY};
use crate::contract::uni_router2::{uni_router2_addr, UNI_ROUTER2};
use crate::contract::{readonly_http_provider, weth_addr};
use crate::models::Auction;
use crate::schema::auction::dsl::auction;
use crate::schema::auction::{is_delete, is_published};
use crate::static_connection_pool;
use alloy::hex::FromHex;
use alloy::primitives::Address;
use bigdecimal::BigDecimal;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use crate::contract::uni_pair::UNI_PAIR;

mod update_price;

async fn update_price() {
    let mut connection = static_connection_pool
        .get()
        .expect("error in init static_connection_pool")
        .get()
        .expect("error in connection");
    let auctions = auction
        .filter(is_delete.eq(false))
        .filter(is_published.eq(true))
        .select(Auction::as_select())
        .get_results(&mut connection)
        .expect("get auctions failed");

    // let uni_router2 = UNI_ROUTER2::new(uni_router2_addr(), readonly_http_provider());
    let uni_factory = UNI_FACTORY::new(uni_factory_addr().await, readonly_http_provider());
    for mut x in auctions {
        let pair_addr = uni_factory
            .getPair(
                Address::from_hex(x.token_addr).expect("token_addr error"),
                weth_addr(),
            )
            .call()
            .await
            .expect("uni_factory.getPair rpc error")._0;

      let uni_pair = UNI_PAIR::new(pair_addr, readonly_http_provider());
      let reserves = uni_pair.getReserves().call().await.expect("uni_pair rpc error");
      let token0 = uni_pair.token0().call().await.expect("uni_pair rpc error")._0;
      // let token1 = uni_pair.token1().call().await.expect("uni_pair rpc error")._0;
      
      if token0 ==weth_addr(){
        x.latest_price_in_wei = Some(BigDecimal::from((reserves._reserve0/reserves._reserve1).to::<u128>()));
      } else {
        x.latest_price_in_wei = Some(BigDecimal::from((reserves._reserve1/reserves._reserve0).to::<u128>()));
      }
      
      
    }
}
