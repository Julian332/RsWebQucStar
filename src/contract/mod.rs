use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
    RecommendedFiller, WalletFiller,
};
use alloy::providers::{Identity, Provider, ProviderBuilder, ReqwestProvider, RootProvider, WsConnect};
use alloy::rpc::types::TransactionRequest;
use alloy::transports::http::{Client, Http};
use reqwest::Url;
use std::env;
use std::error::Error;
use std::str::FromStr;
use alloy::pubsub::PubSubFrontend;

pub mod pre_sale_slot;
pub mod uni_graph;
mod uni_graph_params;
pub mod uni_router2;
pub mod uni_factory;
pub mod uni_pair;

pub async fn transfer(
    provider: FillProvider<
        JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>,
        ReqwestProvider,
        Http<Client>,
        Ethereum,
    >,
    to: Address,
    value: U256,
) -> Result<TxHash, Box<dyn Error>> {
    // let provider: FillProvider<JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>, ReqwestProvider, Http<Client>, Ethereum> = ProviderBuilder::new()
    //   .with_recommended_fillers()
    //   .wallet(ethereum_wallet)
    //   .on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());

    let tx = TransactionRequest::default().with_to(to).with_value(value);
    let builder = provider.send_transaction(tx).await?;
    let tx_hash = builder.tx_hash();
    Ok(*tx_hash)
}

pub async fn transfer_with_nonce(
    provider: FillProvider<
        JoinFill<RecommendedFiller, WalletFiller<EthereumWallet>>,
        ReqwestProvider,
        Http<Client>,
        Ethereum,
    >,
    to: Address,
    value: U256,
    nonce: u64,
) -> Result<TxHash, Box<dyn Error>> {
    // let provider = ProviderBuilder::new()
    //   .with_gas_estimation()
    //   .filler(ChainIdFiller::default())    // .with_recommended_fillers()
    //   .wallet(ethereum_wallet)
    //   .on_http(Url::from_str(env::var("ETH_RPC")?.as_str()).unwrap());

    let tx = TransactionRequest::default()
        .with_to(to)
        .with_nonce(nonce)
        .with_value(value);
    let builder = provider.send_transaction(tx).await?;
    let tx_hash = builder.tx_hash();
    Ok(*tx_hash)
}
pub fn readonly_http_provider() -> FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    ReqwestProvider,
    Http<Client>,
    Ethereum,
> {
    ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(Url::from_str(env::var("ETH_RPC").expect(".env ETH_RPC").as_str()).unwrap())
}

pub async fn readonly_ws_provider() -> RootProvider<PubSubFrontend> {
    let rpc_url = env::var("WS_ETH_RPC").unwrap();
    let ws = WsConnect::new(rpc_url);
    ProviderBuilder::new().on_ws(ws).await.unwrap()
}

pub fn weth_addr() -> Address {
    Address::from_str(env::var("WETH_ADDR").expect(".env WETH_ADDR").as_str()).expect(".env WETH_ADDR")
}