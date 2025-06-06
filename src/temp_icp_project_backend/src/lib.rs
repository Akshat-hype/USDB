use ic_cdk::{api::{caller, canister_balance}, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use candid::Principal;

type UsdbAmount = u64;

// === Token Metadata ===
const TOKEN_NAME: &str = "US Dollar Bitcoin";
const TOKEN_SYMBOL: &str = "USDB";
const DECIMALS: u8 = 8; // Like BTC/ICP
const OWNER: &str = "token_owner"; // Replace with real owner principal if needed

#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserBalance {
    pub principal: Principal,
    pub amount: UsdbAmount,
}

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<Vec<UserBalance>> = RefCell::new(Vec::new());
}

/// Greetings
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

/// Token Metadata
#[query]
fn get_token_name() -> String {
    TOKEN_NAME.to_string()
}

#[query]
fn get_token_symbol() -> String {
    TOKEN_SYMBOL.to_string()
}

#[query]
fn get_decimals() -> u8 {
    DECIMALS
}

#[query]
fn get_token_owner() -> Principal {
    Principal::from_text(OWNER).unwrap_or_else(|_| Principal::anonymous())
}

/// Total supply of USDB
#[query]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

/// Mint USDB to caller
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

/// Burn USDB from caller
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

/// Caller’s balance
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

/// Transfer USDB to another principal
#[update]
fn transfer_usdb(to: Principal, amount: UsdbAmount) -> String {
    let from = caller();

    if amount == 0 {
        ic_cdk::trap("Transfer amount must be greater than 0.");
    }

    if from == to {
        ic_cdk::trap("Cannot transfer to self.");
    }

    USER_BALANCES.with(|balances| {
        let mut balances = balances.borrow_mut();

        // Find sender
        let sender_opt = balances.iter_mut().find(|b| b.principal == from);
        let sender = match sender_opt {
            Some(s) => s,
            None => ic_cdk::trap("Sender has no balance."),
        };

        if sender.amount < amount {
            ic_cdk::trap("Insufficient balance.");
        }

        sender.amount -= amount;

        // Find or create recipient
        match balances.iter_mut().find(|b| b.principal == to) {
            Some(receiver) => receiver.amount += amount,
            None => balances.push(UserBalance {
                principal: to,
                amount,
            }),
        }
    });

    format!("Transferred {} USDB to {}", amount, to)
}

/// Get current canister cycle balance
#[query]
fn get_cycles() -> u128 {
    canister_balance()
}
