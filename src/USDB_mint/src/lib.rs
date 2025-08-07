use ic_cdk::{api::caller, query, update};
use ic_cdk_macros::{init, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;

type TokenAmount = u64;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, TokenAmount>> = RefCell::new(HashMap::new());
    static OWNER: RefCell<Option<Principal>> = RefCell::new(None);
    static TOTAL_SUPPLY: RefCell<TokenAmount> = RefCell::new(0);
}

#[init]
fn init() {
    OWNER.with(|o| *o.borrow_mut() = Some(caller()));
}

fn is_owner() -> bool {
    OWNER.with(|o| o.borrow().map_or(false, |p| p == caller()))
}

#[update]
fn mint(to: Principal, amount: TokenAmount) -> String {
    if !is_owner() {
        ic_cdk::trap("Only the owner can mint tokens.");
    }

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        *balances.entry(to).or_insert(0) += amount;
    });

    TOTAL_SUPPLY.with(|supply| *supply.borrow_mut() += amount);

    format!("Minted {} tokens to {}", amount, to)
}

#[query]
fn get_balance_of(user: Principal) -> TokenAmount {
    BALANCES.with(|b| *b.borrow().get(&user).unwrap_or(&0))
}

#[query]
fn get_my_balance() -> TokenAmount {
    get_balance_of(caller())
}

#[query]
fn get_total_supply() -> TokenAmount {
    TOTAL_SUPPLY.with(|s| *s.borrow())
}
