use ic_cdk::query;
use ic_cdk_macros::*;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
