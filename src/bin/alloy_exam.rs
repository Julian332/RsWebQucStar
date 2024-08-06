//! Example of generating code from ABI file using the `sol!` macro to interact with the contract.

// use alloy::{node_bindings::Anvil, primitives::address, providers::ProviderBuilder, sol};
// use eyre::Result;

use std::error::Error;
use std::str::FromStr;

use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use alloy::sol;
use alloy::transports::http::reqwest::Url;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "src/contract/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
  // Spin up a forked Anvil node.
  // Ensure `anvil` is available in $PATH.
  // let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;
  // 
  // // Create a provider.
  // let rpc_url = anvil.endpoint().parse()?;
  let provider = ProviderBuilder::new().on_http(Url::from_str("https://eth-mainnet.g.alchemy.com/v2/bl_l3tjwjUkNOGgFF_UCer3-UbO5yRYU").unwrap());

  // Create a contract instance.
  let contract = IWETH9::new(address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), provider);

  // Call the contract, retrieve the total supply.
  let total_supply = contract.totalSupply().call().await?._0;

  println!("WETH total supply is {total_supply}");

  Ok(())
}