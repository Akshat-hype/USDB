use ic_cdk::{api::caller, call, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use candid::{CandidType, Principal};

type UsdbAmount = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserBalance {
    principal: Principal,
    amount: UsdbAmount,
}

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Serialize, Deserialize)]
struct MintResponse {
    btc_required: f64,
    btc_price: f64,
    btc_address: String,
}

#[update]
async fn initiate_usdb_mint(amount: UsdbAmount) -> MintResponse {
    let btc_price_canister: Principal = Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").unwrap();

    let (price_str,): (String,) = call(btc_price_canister, "get_btc_price", ()).await.unwrap_or(("0.0".to_string(),));
    let btc_price: f64 = price_str.parse().unwrap_or(0.0);

    if btc_price <= 0.0 {
        ic_cdk::trap("Invalid BTC price received");
    }

    let btc_required = (amount as f64) / btc_price; 

    let btc_address = "tb1qmc3r3vnjtzfj2slehrklehw5vmqr3je8d8wzc6".to_string(); // Use a real/test address here

    MintResponse {
        btc_required,
        btc_price,
        btc_address,
    }
}

#[update]
fn confirm_and_mint(amount: UsdbAmount) -> UsdbAmount {
    let user = caller();

    // This is where you'd verify BTC payment (currently skipped for testing)

    TOTAL_SUPPLY.with(|supply| {
        *supply.borrow_mut() += amount;
    });

    USER_BALANCES.with(|balances| {
        let mut user_balances = balances.borrow_mut();
        if let Some(entry) = user_balances.iter_mut().find(|b| b.principal == user) {
            entry.amount += amount;
        } else {
            user_balances.push(UserBalance {
                principal: user,
                amount,
            });
        }
    });

    amount
}

#[update]
fn burn_usdb(amount: UsdbAmount) -> UsdbAmount {
    let user = caller();

    USER_BALANCES.with(|balances| {
        let mut user_balances = balances.borrow_mut();
        if let Some(entry) = user_balances.iter_mut().find(|b| b.principal == user) {
            if entry.amount >= amount {
                entry.amount -= amount;
                TOTAL_SUPPLY.with(|supply| {
                    *supply.borrow_mut() -= amount;
                });
            } else {
                ic_cdk::trap("Insufficient USDB balance to burn.");
            }
        } else {
            ic_cdk::trap("No USDB balance found for caller.");
        }
    });

    amount
}

#[update]
fn get_my_balance() -> UsdbAmount {
    let user = caller();
    USER_BALANCES.with(|balances| {
        balances
            .borrow()
            .iter()
            .find(|b| b.principal == user)
            .map_or(0, |b| b.amount)
    })
}

#[update]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}
