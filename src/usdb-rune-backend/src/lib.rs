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

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[query]
fn greet(user: String) -> String {
    format!("Hello, {}!", user)
}

#[query]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

#[update]
fn mint_usdb(amount: UsdbAmount) -> UsdbAmount {
    let user = caller();

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

    get_total_supply()
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

    get_total_supply()
}

#[query]
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
