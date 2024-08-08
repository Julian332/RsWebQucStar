use std::error::Error;
use std::str::FromStr;

use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::{Provider, ReqwestProvider};
use alloy::providers::fillers::{FillProvider, JoinFill, RecommendedFiller, WalletFiller};
use alloy::rpc::types::TransactionRequest;
use alloy::transports::http::{Client, Http};

pub mod uni_router2;

pub async fn transfer(provider: FillProvider<JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum>, to: Address, value: U256) -> Result<TxHash, Box<dyn Error>> {
  // let provider: FillProvider<JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum> = ProviderBuilder::new()
  //   .with_recommended_fillers()
  //   .wallet(ethereum_wallet)
  //   .on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());

  let tx = TransactionRequest::default().with_to(to).with_value(value);
  let builder = provider.send_transaction(tx).await?;
  let tx_hash = builder.tx_hash();
  Ok(*tx_hash)
}

pub async fn transfer_with_nonce(provider: FillProvider<JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum>, to: Address, value: U256,nonce:u64) -> Result<TxHash, Box<dyn Error>> {
  // let provider = ProviderBuilder::new()
  //   .with_gas_estimation()
  //   .filler(ChainIdFiller::default())    // .with_recommended_fillers()
  //   .wallet(ethereum_wallet)
  //   .on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());

  let tx = TransactionRequest::default().with_to(to).with_nonce(nonce).with_value(value);
  let builder = provider.send_transaction(tx).await?;
  let tx_hash = builder.tx_hash();
  Ok(*tx_hash)
}