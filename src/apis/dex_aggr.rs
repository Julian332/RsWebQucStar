use std::env;

use alloy::hex;
use alloy::primitives::Address;
use alloy::transports::http::reqwest;
use base64::Engine;
use base64::engine::general_purpose;
use bigdecimal::BigDecimal;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::apis::dex_aggr_resp::DexAggrResp;
use crate::openapi::errors::AppError;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;
pub async fn dex_aggr(chain_id: isize, amount: BigDecimal, to_token_address: Address, from_token_address: Address, slippage: Option<BigDecimal>, user_wallet_address: Address) -> Result<DexAggrResp, Box<dyn std::error::Error>> {
  let path_and_params = format!("/api/v5/dex/aggregator/swap?\
    chainId={}\
    &amount={}\
    &toTokenAddress={}\
    &fromTokenAddress={}\
    &slippage={}\
    &userWalletAddress={}", chain_id, amount, to_token_address, from_token_address, slippage.unwrap_or(BigDecimal::from(0)), user_wallet_address);
  let client = reqwest::Client::builder()
    .build()?;


  let access_project = env::var("OK_ACCESS_PROJECT").expect("OK_ACCESS_PROJECT must be set");
  let access_key = env::var("OK_ACCESS_KEY").expect("OK_ACCESS_KEY must be set");
  let access_passphrase = env::var("OK_ACCESS_PASSPHRASE").expect("OK_ACCESS_PASSPHRASE must be set");
  let now = format!("{}Z", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S.%3f"));
  // let now = chrono::Utc::now(). "2024-08-06T03:38:34.834Z" 2024-08-06T03:09:57.406Z

  let ok_access_secret_key = env::var("OK_ACCESS_SECRET_KEY").expect("OK_ACCESS_SECRET_KEY must be set");

  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert("OK-ACCESS-PROJECT", access_project.parse()?);
  headers.insert("OK-ACCESS-KEY", access_key.parse()?);
  headers.insert("OK-ACCESS-PASSPHRASE", access_passphrase.parse()?);
  headers.insert("OK-ACCESS-TIMESTAMP", now.parse()?);

  // 2024-08-06T03:20:10.906ZGET/api/v5/dex/aggregator/swap?chainId=1&amount=1000000000&toTokenAddress=0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48&fromTokenAddress=0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee&slippage=0.05&userWalletAddress=0x6f9ffea7370310cd0f890dfde5e0e061059dcfb8
  // 2024-08-06T03:48:02.738ZGET/api/v5/dex/aggregator/swap?chainId=1&amount=1000&toTokenAddress=0xA7f6cEd0968Dc9053B23E1A08291f05270FD5888&fromTokenAddress=0xA7f6cEd0968Dc9053B23E1A08291f05270FD5888&slippage=0&userWalletAddress=0xA7f6cEd0968Dc9053B23E1A08291f05270FD5888
  // let bytes = ;
  let mut mac = HmacSha256::new_from_slice(hex::decode(ok_access_secret_key)?.as_slice())
    .expect("HMAC can take key of any size");
  let message = format!("{}{}{}", now, "GET", path_and_params);
  // let message = "2024-08-06T06:31:17ZGET/api/v5/dex/aggregator/quote?chainId=42161&amount=1000000000000&toTokenAddress=0xff970a61a04b1ca14834a43f5de4533ebddb5cc8&fromTokenAddress=0x82aF49447D8a07e3bd95BD0d56f35241523fBab1".to_string();
  mac.update(message.as_bytes());

  let result = &mac.finalize().into_bytes()[..];
  let okx_sign = general_purpose::STANDARD.encode(result);
  // 0Z2NgUIfKxm9C5C2zM5TrneniqsikTBFqfz+j5/pU5s=
  // NZfsSC6tPOVLCBQ+GJdTUwEpJdmP198/bW3nKa4CKDE=
  headers.insert("OK-ACCESS-SIGN", okx_sign.parse()?);
  // `result` has type `CtOutput` which is a thin wrapper around array of
  // bytes for providing constant time equality check
  // To get underlying array use `into_bytes`, but be careful, since
  // incorrect use of the code value may permit timing attacks which defeats
  // the security provided by the `CtOutput`
  // let code_bytes = result;
  //   let expected = hex!("
  //     97d2a569059bbcd8ead4444ff99071f4
  //     c01d005bcefe0d3567e1be628e5fdcd9
  // ");
  //   assert_eq!(code_bytes[..], expected[..]);


  let domain = "https://www.okx.com".to_string();

  let request = client.request(reqwest::Method::GET, format!("{}{}", domain, path_and_params));


  let request = request.headers(headers);

  let response = request.send().await?;
  let body = response.text().await?;
  match serde_json::from_str::<DexAggrResp>(body.as_str()) {
    Ok(x) => { Ok(x) }
    Err(e) => { Err(Box::new(AppError::new(e.to_string().as_str()))) }
  }
}