
// TradingOrder
// Created by julian on 2024-07-31.
#[derive(Default, Debug)]
pub struct TradingOrder {
    pub id: String, // id
    pub deleted: String, // deleted
    pub create_time: String, // create_time
    pub update_time: String, // update_time
    pub sell_or_buy: String, // sell|buy
    pub target_token: String, // target_token
    pub from_token: String, // from_token
    pub trading_uer: String, // trading_uer
    pub boost_mode: String, // boost_mode
    pub mev_protected: String, // mev_protected
    pub priority_fee: u32, // priority_fee
    pub is_succeed: String, // is_successed
    pub tx_hash: String, // tx_hash
    pub tx_receipt: String, // tx_receipt
    pub target_amount: u32, // target_amount
    pub from_token_amount: u32, // from_token_amount
    pub order_type: String, // trading|pending|following
    pub pending_target_price: u32, // pending_target_price
    pub expire_at: String, // expire_at
    pub fee: u32, // fee
}