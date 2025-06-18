use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpHeader,
};
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    static BTC_PRICE: RefCell<Option<f64>> = RefCell::new(None);
}

#[derive(Deserialize)]
struct TaapiResponse {
    value: f64,
}

#[update]
async fn fetch_btc_price() -> String {
    let url = "https://api.taapi.io/price?secret=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjbHVlIjoiNjVjYzg5MzIxNDBjZmQ3MjNkYTlhMTZmIiwiaWF0IjoxNzA3OTAzMjgyLCJleHAiOjMzMjEyMzY3MjgyfQ.26yhTp17R-A9duiLhSmsOk3BhkCv72fo0Oolmy2aAtA&exchange=binance&symbol=BTC/USDT&interval=1m";

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        }],
        body: None,
        max_response_bytes: Some(2048),
        transform: None,
    };

    match http_request(request).await {
        Ok((response,)) => {
            if response.status == 200 {
                if let Ok(result) = serde_json::from_slice::<TaapiResponse>(&response.body) {
                    BTC_PRICE.with(|p| *p.borrow_mut() = Some(result.value));
                    format!("Fetched BTC price: {}", result.value)
                } else {
                    "Failed to parse price".to_string()
                }
            } else {
                format!("HTTP error: {}", response.status)
            }
        }
        Err(e) => format!("HTTP call failed: {:?}", e),
    }
}

#[query]
fn get_last_price() -> Option<f64> {
    BTC_PRICE.with(|p| *p.borrow())
}

