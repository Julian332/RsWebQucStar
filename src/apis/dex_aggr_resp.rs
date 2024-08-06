// YApi QuickType插件生成，具体参考文档:https://plugins.jetbrains.com/plugin/18847-yapi-quicktype/documentation

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DexAggrResp {
    #[serde(rename = "msg")]
    pub msg: String,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "data")]
    pub data: Vec<Datum>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "routerResult")]
    router_result: RouterResult,

    #[serde(rename = "tx")]
    tx: Tx,
}

#[derive(Serialize, Deserialize)]
pub struct RouterResult {
    #[serde(rename = "toTokenAmount")]
    to_token_amount: String,

    #[serde(rename = "fromTokenAmount")]
    from_token_amount: String,

    #[serde(rename = "chainId")]
    chain_id: String,

    #[serde(rename = "dexRouterList")]
    dex_router_list: Vec<DexRouterList>,

    #[serde(rename = "estimateGasFee")]
    estimate_gas_fee: String,

    #[serde(rename = "quoteCompareList")]
    quote_compare_list: Vec<QuoteCompareList>,

    #[serde(rename = "fromToken")]
    from_token: Token,

    #[serde(rename = "toToken")]
    to_token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct DexRouterList {
    #[serde(rename = "router")]
    router: String,

    #[serde(rename = "subRouterList")]
    sub_router_list: Vec<SubRouterList>,

    #[serde(rename = "routerPercent")]
    router_percent: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubRouterList {
    #[serde(rename = "dexProtocol")]
    dex_protocol: Vec<DexProtocol>,

    #[serde(rename = "fromToken")]
    from_token: Token,

    #[serde(rename = "toToken")]
    to_token: Token,
}

#[derive(Serialize, Deserialize)]
pub struct DexProtocol {
    #[serde(rename = "dexName")]
    dex_name: String,

    #[serde(rename = "percent")]
    percent: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "tokenUnitPrice")]
    token_unit_price: String,

    #[serde(rename = "tokenSymbol")]
    token_symbol: String,

    #[serde(rename = "tokenContractAddress")]
    token_contract_address: String,

    #[serde(rename = "decimal")]
    decimal: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuoteCompareList {
    #[serde(rename = "dexLogo")]
    dex_logo: String,

    #[serde(rename = "tradeFee")]
    trade_fee: String,

    #[serde(rename = "amountOut")]
    amount_out: String,

    #[serde(rename = "dexName")]
    dex_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tx {
    #[serde(rename = "data")]
    data: String,

    #[serde(rename = "isEstimateError")]
    is_estimate_error: bool,

    #[serde(rename = "gas")]
    gas: String,

    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: String,

    #[serde(rename = "minReceiveAmount")]
    min_receive_amount: String,

    #[serde(rename = "from")]
    from: String,

    #[serde(rename = "to")]
    to: String,

    #[serde(rename = "estimateErrorMsg")]
    estimate_error_msg: String,

    #[serde(rename = "value")]
    value: String,

    #[serde(rename = "gasPrice")]
    gas_price: String,
}
