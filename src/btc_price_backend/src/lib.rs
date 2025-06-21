use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use ic_cdk::update;
use std::cell::RefCell;

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
        headers: vec![],
        body: None,
        transform: None,
    };

    let cycles: u128 = 5_000_000_000;

    // Perform the HTTP request
    if let Ok((response,)) = http_request(request, cycles).await {
        if let Ok(body_str) = String::from_utf8(response.body) {
            // crude JSON extraction without serde_json
            if let Some(price_val) = extract_price(&body_str) {
                BTC_PRICE.with(|p| p.replace(price_val.clone()));
                return price_val;
            } else {
                ic_cdk::println!("Failed to extract price from response");
            }
        } else {
            ic_cdk::println!("Failed to parse response body");
        }
    } else {
        ic_cdk::println!("HTTP request failed");
    }

    // fallback to last known value
    BTC_PRICE.with(|p| p.borrow().clone())
}

/// crude string-based extractor to get value from: {"symbol":"BTCUSDT","price":"67123.45"}
fn extract_price(json_str: &str) -> Option<String> {
    // look for: "price":" and extract till next "
    let key = "\"price\":\"";
    if let Some(start) = json_str.find(key) {
        let from = start + key.len();
        if let Some(end) = json_str[from..].find('"') {
            return Some(json_str[from..from + end].to_string());
        }
    }
    None
}
