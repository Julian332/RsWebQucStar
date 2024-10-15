//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use std::error::Error;
use std::str::FromStr;

use crate::contract::readonly_http_provider;
use crate::contract::uni_factory::{uni_factory_addr, UNI_FACTORY};

use alloy::primitives::Address;

use alloy::sol;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UNI_PAIR,
    "src/contract/uni_pair.json"
);

pub async fn get_pair(token_a: Address, token_b: Address) -> Address {
    let uni_factory = UNI_FACTORY::new(uni_factory_addr().await, readonly_http_provider());
    uni_factory
        .getPair(token_a, token_b)
        .call()
        .await
        .expect("uni_factory.getPair rpc error")
        ._0
}
