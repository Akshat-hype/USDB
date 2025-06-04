use ic_cdk::{api::caller, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use candid::Principal;

type UsdbAmount = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserBalance {
    pub principal: Principal,
    pub amount: UsdbAmount,
}

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// Returns the total minted supply of USDB.
#[query]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

/// Mints `amount` of USDB for the caller.
/// NOTE: In production, this should validate BTC collateral before minting.
#[update]
fn mint_usdb(amount: UsdbAmount) -> UsdbAmount {
    let minter = caller();

    TOTAL_SUPPLY.with(|supply| {
        *supply.borrow_mut() += amount;
    });

    USER_BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();
        match balances.iter_mut().find(|b| b.principal == minter) {
            Some(user) => user.amount += amount,
            None => balances.push(UserBalance {
                principal: minter,
                amount,
            }),
        }
    });

    get_total_supply()
}

/// Burns `amount` of USDB from the caller's balance.
/// NOTE: In production, this should trigger BTC redemption.
#[update]
fn burn_usdb(amount: UsdbAmount) -> UsdbAmount {
    let burner = caller();

    let mut user_found = false;

    USER_BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        if let Some(user) = balances.iter_mut().find(|b| b.principal == burner) {
            if user.amount >= amount {
                user.amount -= amount;
                user_found = true;

                TOTAL_SUPPLY.with(|supply| {
                    *supply.borrow_mut() -= amount;
                });
            } else {
                ic_cdk::trap("Insufficient USDB balance to burn.");
            }
        }
    });

    if !user_found {
        ic_cdk::trap("No USDB balance found for caller.");
    }

    get_total_supply()
}

/// Returns the caller's current USDB balance.
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
