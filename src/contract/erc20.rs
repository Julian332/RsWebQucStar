use alloy::hex::FromHex;
use alloy::sol;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use futures_util::StreamExt;
use std::str::FromStr;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC_20,
    "src/contract/erc20.json"
);
