use candid::{CandidType, Principal, candid_method};
use ic_cdk_macros::update;
use serde::Deserialize;
use std::cell::RefCell;

type UsdbAmount = u64;

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Deserialize, Clone)]
struct UserBalance {
    principal: Principal,
    amount: UsdbAmount,
}

#[derive(CandidType, Deserialize)]
struct BtcDepositNotification {
    btc_address: String,
    txid: String,
    amount_sats: u64,
    usdb_amount: UsdbAmount,
    receiver: Principal,
}

#[update]
#[candid_method(update)]
fn notify_btc_received(notification: BtcDepositNotification) -> String {
    let BtcDepositNotification {
        receiver,
        usdb_amount,
        ..
    } = notification;

    // Increase total supply
    TOTAL_SUPPLY.with(|supply| {
        *supply.borrow_mut() += usdb_amount;
    });

    // Update user balance
    USER_BALANCES.with(|balances| {
        let mut user_balances = balances.borrow_mut();
        if let Some(entry) = user_balances.iter_mut().find(|b| b.principal == receiver) {
            entry.amount += usdb_amount;
        } else {
            user_balances.push(UserBalance {
                principal: receiver,
                amount: usdb_amount,
            });
        }
    });

    format!("✅ Minted {} USDB to {}", usdb_amount, receiver.to_text())
}
