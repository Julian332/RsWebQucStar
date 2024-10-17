use crate::contracts::uni_pair::{get_pair, UNI_PAIR};
use crate::contracts::usdt::usdt_addr;
use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::primitives::aliases::U112;
use alloy::primitives::{Address, TxHash, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
    RecommendedFiller, WalletFiller,
};
use alloy::providers::{
    Identity, Provider, ProviderBuilder, ReqwestProvider, RootProvider, WsConnect,
};
use alloy::pubsub::PubSubFrontend;
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use reqwest::Url;
use std::env;
use std::error::Error;
use std::str::FromStr;

mod erc20;
pub mod uni_factory;
pub mod uni_pair;
pub mod uni_router2;
mod usdt;

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
    Address::from_str(env::var("WETH_ADDR").expect(".env WETH_ADDR").as_str())
        .expect(".env WETH_ADDR")
}

pub async fn get_dollar_price<T: Into<U112>>(x: T) -> u128 {
    let usdt_pair = UNI_PAIR::new(
        get_pair(usdt_addr(), weth_addr()).await,
        readonly_http_provider(),
    );
    let reserves_return = usdt_pair
        .getReserves()
        .call()
        .await
        .expect("reserves failed");

    let token0 = usdt_pair.token0().call().await.expect("token0 failed")._0;
    if token0 == weth_addr() {
        reserves_return._reserve1 * x.into() / reserves_return._reserve0
    } else {
        reserves_return._reserve0 * x.into() / reserves_return._reserve1
    }
    .to::<u128>()
}
pub fn get_project_signer() -> EthereumWallet {
    let project_pk = env::var("PROJECT_SIGNER").expect(".env PROJECT_SIGNER");
    let signer = PrivateKeySigner::from_str(&project_pk).expect(".env PROJECT_SIGNER");
    EthereumWallet::new(signer)
}

pub fn signer_http_provider() -> FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    ReqwestProvider,
    Http<Client>,
    Ethereum,
> {
    ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(get_project_signer())
        .on_http(eth_http_rpc())
}

pub fn eth_http_rpc() -> Url {
    Url::from_str(env::var("ETH_RPC").expect(".env ETH_RPC").as_str()).unwrap()
}
