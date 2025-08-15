use candid::{CandidType, Principal};
use serde::Deserialize;
use std::cell::RefCell;

// Define UsdbAmount as a type alias for u64
type UsdbAmount = u64;

#[derive(CandidType, Deserialize)]
struct BtcDepositNotification {
    btc_address: String,
    txid: String,
    amount_sats: u64,
    usdb_amount: UsdbAmount,
    receiver: Principal,
}
#[derive(Clone)]
#[allow(dead_code)]
struct UserBalance {
    principal: Principal,
    amount: UsdbAmount,
}

// Static variable for total supply
thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[allow(dead_code)]
// #[update]
fn notify_btc_received(notification: BtcDepositNotification) -> String {
    let BtcDepositNotification {
        receiver,
        usdb_amount,
        ..
    } = notification;

    // Mint tokens to the user
    TOTAL_SUPPLY.with(|supply| {
        *supply.borrow_mut() += usdb_amount;
    });

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
