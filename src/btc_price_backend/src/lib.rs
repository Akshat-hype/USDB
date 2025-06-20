use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use ic_cdk::update;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

thread_local! {
    static BTC_PRICE: RefCell<String> = RefCell::new("0.0".to_string());
}

#[update]
async fn get_btc_price() -> String {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(2048),
        method: HttpMethod::GET,
        headers: vec![], // No API key needed for this public endpoint
        body: None,
        transform: None,
    };

    let cycles: u128 = 5_000_000_000;

    // Fetch latest price
    if let Ok((response,)) = http_request(request, cycles).await {
        if let Ok(body_str) = String::from_utf8(response.body) {
            BTC_PRICE.with(|p| p.replace(body_str.clone()));
            return body_str;
        } else {
            ic_cdk::println!("Failed to parse response body");
        }
    } else {
        ic_cdk::println!("HTTP request failed");
    }

    // Fallback: return last stored value
    BTC_PRICE.with(|p| p.borrow().clone())
}
