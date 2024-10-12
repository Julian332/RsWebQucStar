//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use std::env;
use std::error::Error;
use std::str::FromStr;

use crate::contract::readonly_http_provider;
use crate::contract::uni_router2::UNI_ROUTER2::UNI_ROUTER2Instance;
use crate::contract::uni_router2::{uni_router2_addr, UNI_ROUTER2};
use alloy::network::{Ethereum, EthereumWallet};
use alloy::primitives::Address;
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::providers::ReqwestProvider;
use alloy::sol;
use alloy::transports::http::{Client, Http};

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_FACTORY,
    "src/contract/uni_factory.json"
);

pub async fn uni_factory_addr() -> Address {
    let uni_router2 = UNI_ROUTER2::new(uni_router2_addr(), readonly_http_provider());
    uni_router2
        .factory()
        .call()
        .await
        .expect("uni_router2.factory() RPC call failed")
        ._0
}
