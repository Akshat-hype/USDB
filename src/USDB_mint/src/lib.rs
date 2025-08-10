use ic_cdk::{api::caller, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use candid::Principal;

type UsdbAmount = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserBalance {
    principal: Principal,
    amount: UsdbAmount,
}

#[derive(Serialize, Deserialize, Debug)]
struct MintResult {
    total_supply: UsdbAmount,
    user_balance: UsdbAmount,
}

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

#[query]
fn get_my_balance() -> UsdbAmount {
    let current = caller();
    USER_BALANCES.with(|balances| {
        balances
            .borrow()
            .iter()
            .find(|b| b.principal == current)
            .map_or(0, |b| b.amount)
    })
}

#[update]
fn mint_usdb(amount: UsdbAmount) -> MintResult {
    if amount == 0 {
        ic_cdk::trap("Amount must be greater than zero");
    }

    let user = caller();

    TOTAL_SUPPLY.with(|supply| {
        *supply.borrow_mut() += amount;
    });

    USER_BALANCES.with(|user_balances| {
        let mut balances = user_balances.borrow_mut();
        if let Some(entry) = balances.iter_mut().find(|b| b.principal == user) {
            entry.amount += amount;
        } else {
            balances.push(UserBalance {
                principal: user,
                amount,
            });
        }
    });

    ic_cdk::println!("Minted {} USDB for {}", amount, user);

    MintResult {
        total_supply: get_total_supply(),
        user_balance: get_my_balance(),
    }
}

#[update]
fn burn_usdb(amount: UsdbAmount) -> MintResult {
    if amount == 0 {
        ic_cdk::trap("Amount must be greater than zero");
    }

    let user = caller();

    USER_BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        if let Some(entry) = balances.iter_mut().find(|b| b.principal == user) {
            if let Some(new_amount) = entry.amount.checked_sub(amount) {
                entry.amount = new_amount;
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

    ic_cdk::println!("Burned {} USDB from {}", amount, user);

    MintResult {
        total_supply: get_total_supply(),
        user_balance: get_my_balance(),
    }
}

#[update]
fn transfer_usdb(to: Principal, amount: UsdbAmount) {
    if amount == 0 {
        ic_cdk::trap("Amount must be greater than zero");
    }

    let from = caller();

    // Deduct from sender
    USER_BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        let sender_balance = balances
            .iter_mut()
            .find(|b| b.principal == from)
            .unwrap_or_else(|| ic_cdk::trap("No balance found for sender"));

        if let Some(new_amount) = sender_balance.amount.checked_sub(amount) {
            sender_balance.amount = new_amount;
        } else {
            ic_cdk::trap("Insufficient balance for transfer");
        }

        // Add to recipient
        if let Some(receiver) = balances.iter_mut().find(|b| b.principal == to) {
            receiver.amount += amount;
        } else {
            balances.push(UserBalance {
                principal: to,
                amount,
            });
        }
    });

    ic_cdk::println!("Transferred {} USDB from {} to {}", amount, from, to);
}
