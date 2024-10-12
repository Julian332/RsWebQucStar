use crate::models::Auction;
use crate::schema::auction::dsl::auction;
use crate::schema::auction::{is_delete, token_addr};
use alloy::hex::FromHex;
use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, RootProvider, WsConnect};
use alloy::pubsub::PubSubFrontend;
use alloy::sol;
use bigdecimal::BigDecimal;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use futures_util::StreamExt;
use std::env;
use tracing::info;
use crate::contract::readonly_ws_provider;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PRE_SALE_SLOT,
    "src/contract/pre_sale_slot.json"
);

pub async fn subscribe_publishing(pool: Pool<ConnectionManager<PgConnection>>) {
    info!("subscribe_publishing started");
    let mut connection = pool.get().unwrap();

    let pre_sale_addr = Address::from_hex(env::var("PRE_SALE_SLOT_ADDR").unwrap()).unwrap();

    let provider = readonly_ws_provider().await;

    let pre_sale_instance = PRE_SALE_SLOT::new(pre_sale_addr, provider);

    let poller = pre_sale_instance
        .Published_filter()
        .watch()
        .await
        .expect("poll pre_sale_slot failed");

    // Subscribe to logs.
    let mut stream = poller
        // .with_poll_interval(Duration::from_secs(6))
        .into_stream();

    while let Some(log) = stream.next().await {
        let (published, log) = log.expect("log.expect");

        let mut auct = auction
            .filter(is_delete.eq(false))
            .filter(token_addr.eq(published.token.to_string()))
            .select(Auction::as_select())
            .first(&mut connection)
            .expect("get auctions failed");
        auct.is_published = true;
        let succeed_users = published.succeedUsers.to_vec();
        let last_succeed_user = succeed_users.chunks_exact(20).last();
        match last_succeed_user {
            None => {
                auct.published_price_in_wei = None;
            }
            Some(x) => {
                let last_succeed_user = Address::from_slice(x);
                let last_succeed_user_bid = pre_sale_instance
                    ._bids(published.token, last_succeed_user)
                    .call()
                    .await
                    .expect("get bids failed");
                auct.published_price_in_wei =
                    Some(BigDecimal::from(last_succeed_user_bid._0.to::<u128>()));
            }
        }
        diesel::update(auction.find(auct.id))
            .set(auct)
            .execute(&mut connection)
            .expect("update auction failed");
    }
}

// pub async fn readonly_ws_provider() -> RootProvider<PubSubFrontend> {
//     let rpc_url = env::var("WS_ETH_RPC").unwrap();
//     let ws = WsConnect::new(rpc_url);
//     ProviderBuilder::new().on_ws(ws).await.unwrap()
// }
