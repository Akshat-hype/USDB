// USDB Token Canister with advanced features
dfn main() {}

use ic_cdk::{api::{caller, canister_balance}, query, update, init};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;

// === Token Metadata ===
const TOKEN_NAME: &str = "US Dollar Bitcoin";
const TOKEN_SYMBOL: &str = "USDB";
const DECIMALS: u8 = 8;

type UsdbAmount = u64;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TransferEvent {
    from: Principal,
    to: Principal,
    amount: UsdbAmount,
    timestamp: u64,
}

thread_local! {
    static TOTAL_SUPPLY: RefCell<UsdbAmount> = RefCell::new(0);
    static USER_BALANCES: RefCell<HashMap<Principal, UsdbAmount>> = RefCell::new(HashMap::new());
    static OWNER: RefCell<Option<Principal>> = RefCell::new(None);
    static TRANSFER_LOG: RefCell<Vec<TransferEvent>> = RefCell::new(Vec::new());
    static ALLOWANCES: RefCell<HashMap<(Principal, Principal), UsdbAmount>> = RefCell::new(HashMap::new());
    static PAUSED: RefCell<bool> = RefCell::new(false);
    static INITIAL_CYCLES: RefCell<u128> = RefCell::new(0);
}

#[init]
fn init() {
    let owner = caller();
    OWNER.with(|o| *o.borrow_mut() = Some(owner));
    INITIAL_CYCLES.with(|c| *c.borrow_mut() = canister_balance());
}

fn is_owner() -> bool {
    OWNER.with(|o| o.borrow().map_or(false, |p| p == caller()))
}

fn check_not_paused() {
    PAUSED.with(|p| {
        if *p.borrow() {
            ic_cdk::trap("Token operations are paused");
        }
    });
}

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
fn get_token_owner() -> Option<Principal> {
    OWNER.with(|o| *o.borrow())
}

#[query]
fn get_total_supply() -> UsdbAmount {
    TOTAL_SUPPLY.with(|supply| *supply.borrow())
}

#[query]
fn get_my_balance() -> UsdbAmount {
    get_balance_of(caller())
}

#[query]
fn get_balance_of(account: Principal) -> UsdbAmount {
    USER_BALANCES.with(|b| *b.borrow().get(&account).unwrap_or(&0))
}

#[update]
fn mint_usdb(amount: UsdbAmount) {
    if !is_owner() {
        ic_cdk::trap("Only owner can mint");
    }
    check_not_paused();
    let to = caller();

    TOTAL_SUPPLY.with(|supply| *supply.borrow_mut() += amount);
    USER_BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        *balances.entry(to).or_insert(0) += amount;
    });
}

#[update]
fn burn_usdb(amount: UsdbAmount) {
    check_not_paused();
    let from = caller();

    USER_BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let balance = balances.entry(from).or_insert(0);
        if *balance < amount {
            ic_cdk::trap("Insufficient balance");
        }
        *balance -= amount;
    });

    TOTAL_SUPPLY.with(|supply| *supply.borrow_mut() -= amount);
}

#[update]
fn transfer_usdb(to: Principal, amount: UsdbAmount) {
    check_not_paused();
    let from = caller();

    if from == to || amount == 0 {
        ic_cdk::trap("Invalid transfer");
    }

    USER_BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let sender_balance = balances.entry(from).or_insert(0);
        if *sender_balance < amount {
            ic_cdk::trap("Insufficient balance");
        }
        *sender_balance -= amount;
        *balances.entry(to).or_insert(0) += amount;
    });

    TRANSFER_LOG.with(|log| {
        log.borrow_mut().push(TransferEvent {
            from,
            to,
            amount,
            timestamp: ic_cdk::api::time(),
        });
    });
}

#[query]
fn get_transfer_log() -> Vec<TransferEvent> {
    TRANSFER_LOG.with(|log| log.borrow().clone())
}

#[update]
fn approve(spender: Principal, amount: UsdbAmount) {
    check_not_paused();
    ALLOWANCES.with(|a| {
        a.borrow_mut().insert((caller(), spender), amount);
    });
}

#[query]
fn allowance(owner: Principal, spender: Principal) -> UsdbAmount {
    ALLOWANCES.with(|a| *a.borrow().get(&(owner, spender)).unwrap_or(&0))
}

#[update]
fn transfer_from(from: Principal, to: Principal, amount: UsdbAmount) {
    check_not_paused();
    let spender = caller();

    ALLOWANCES.with(|a| {
        let mut allowances = a.borrow_mut();
        let key = (from, spender);
        let allowed = allowances.get(&key).cloned().unwrap_or(0);
        if allowed < amount {
            ic_cdk::trap("Allowance too low");
        }
        allowances.insert(key, allowed - amount);
    });

    USER_BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.entry(from).or_insert(0);
        if *from_balance < amount {
            ic_cdk::trap("From balance too low");
        }
        *from_balance -= amount;
        *balances.entry(to).or_insert(0) += amount;
    });

    TRANSFER_LOG.with(|log| {
        log.borrow_mut().push(TransferEvent {
            from,
            to,
            amount,
            timestamp: ic_cdk::api::time(),
        });
    });
}

#[update]
fn pause() {
    if !is_owner() {
        ic_cdk::trap("Only owner can pause");
    }
    PAUSED.with(|p| *p.borrow_mut() = true);
}

#[update]
fn unpause() {
    if !is_owner() {
        ic_cdk::trap("Only owner can unpause");
    }
    PAUSED.with(|p| *p.borrow_mut() = false);
}

#[query]
fn is_paused() -> bool {
    PAUSED.with(|p| *p.borrow())
}

#[query]
fn get_cycles() -> u128 {
    canister_balance()
}

#[query]
fn get_cycles_used() -> u128 {
    INITIAL_CYCLES.with(|c| c.borrow().saturating_sub(canister_balance()))
}
