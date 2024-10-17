use alloy::hex::FromHex;
use alloy::primitives::Address;
use alloy::sol;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use futures_util::StreamExt;
use std::env;
use std::str::FromStr;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    USDT,
    "src/contracts/abis/usdt.json"
);

pub fn usdt_addr() -> Address {
    Address::from_str(env::var("USDT_ADDR").expect(".env USDT_ADDR").as_str())
        .expect(".env USDT_ADDR")
}
