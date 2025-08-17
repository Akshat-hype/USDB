use ic_cdk::{
    api::{caller, canister_balance, time},
    query, update, init, trap,
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use candid::{Principal, CandidType};

// === Token Metadata ===
const TOKEN_NAME: &str = "US Dollar Bitcoin";
const TOKEN_SYMBOL: &str = "USDB";
const DECIMALS: u8 = 8;
type UsdbAmount = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
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
    INITIAL_CYCLES.with(|c| *c.borrow_mut() = canister_balance() as u128);
}

fn is_owner() -> bool {
    OWNER.with(|o| o.borrow().map_or(false, |p| p == caller()))
}

fn check_not_paused() {
    PAUSED.with(|p| {
        if *p.borrow() {
            trap("Token operations are paused");
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
    TOTAL_SUPPLY.with(|s| *s.borrow())
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
fn mint_usdb_to(to: Principal, amount: UsdbAmount) {
    if !is_owner() { trap("Only owner can mint"); }
    check_not_paused();
    TOTAL_SUPPLY.with(|s| *s.borrow_mut() += amount);
    USER_BALANCES.with(|b| {
        let mut bal = b.borrow_mut();
        *bal.entry(to).or_insert(0) += amount;
    });
}

#[update]
fn mint_usdb(amount: UsdbAmount) {
    let me = caller();
    mint_usdb_to(me, amount);
}

#[update]
fn burn_usdb(amount: UsdbAmount) {
    check_not_paused();
    let from = caller();
    USER_BALANCES.with(|b| {
        let mut bal = b.borrow_mut();
        let entry = bal.entry(from).or_insert(0);
        if *entry < amount { trap("Insufficient balance"); }
        *entry -= amount;
    });
    TOTAL_SUPPLY.with(|s| *s.borrow_mut() -= amount);
}

#[update]
fn transfer_usdb(to: Principal, amount: UsdbAmount) {
    check_not_paused();
    let from = caller();
    if from == to || amount == 0 { trap("Invalid transfer"); }
    USER_BALANCES.with(|b| {
        let mut bal = b.borrow_mut();
        let from_bal = bal.entry(from).or_insert(0);
        if *from_bal < amount { trap("Insufficient balance"); }
        *from_bal -= amount;
        *bal.entry(to).or_insert(0) += amount;
    });
    TRANSFER_LOG.with(|log| {
        log.borrow_mut().push(TransferEvent {
            from,
            to,
            amount,
            timestamp: time(),
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
    ALLOWANCES.with(|a| { a.borrow_mut().insert((caller(), spender), amount); });
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
        let mut al = a.borrow_mut();
        let key = (from, spender);
        let allowed = al.get(&key).copied().unwrap_or(0);
        if allowed < amount { trap("Allowance too low"); }
        al.insert(key, allowed - amount);
    });
    USER_BALANCES.with(|b| {
        let mut bal = b.borrow_mut();
        let from_bal = bal.entry(from).or_insert(0);
        if *from_bal < amount { trap("From balance too low"); }
        *from_bal -= amount;
        *bal.entry(to).or_insert(0) += amount;
    });
    TRANSFER_LOG.with(|log| {
        log.borrow_mut().push(TransferEvent {
            from,
            to,
            amount,
            timestamp: time(),
        });
    });
}

#[update]
fn pause() {
    if !is_owner() { trap("Only owner can pause"); }
    PAUSED.with(|p| *p.borrow_mut() = true);
}
#[update]
fn unpause() {
    if !is_owner() { trap("Only owner can unpause"); }
    PAUSED.with(|p| *p.borrow_mut() = false);
}
#[query]
fn is_paused() -> bool {
    PAUSED.with(|p| *p.borrow())
}

#[query]
fn get_cycles() -> u128 {
    canister_balance().into()
}
#[query]
fn get_cycles_used() -> u128 {
    INITIAL_CYCLES.with(|c| c.borrow().saturating_sub(canister_balance().into()))
}

/// Convert Unix epoch millis to an RFC3339 datetime string
#[query]
fn to_readable_timestamp(ts: u64) -> String {
    // Requires the "time" crate with the "formatting" and "macros" features enabled in Cargo.toml:
    // time = { version = "0.3", features = ["formatting", "macros"] }
    use core::time::{OffsetDateTime, format_description::well_known::Rfc3339};
    OffsetDateTime::from_unix_timestamp((ts / 1000) as i64)
        .map(|dt| dt.format(&Rfc3339).unwrap_or_else(|_| "Invalid timestamp".to_string()))
        .unwrap_or_else(|_| "Invalid timestamp".to_string())
}

/// Export balances as CSV: account,amount
#[query]
fn export_balances_csv() -> String {
    let mut csv = "account,amount\n".to_string();
    USER_BALANCES.with(|b| {
        for (acct, bal) in b.borrow().iter() {
            csv += &format!("{},{}\n", acct, bal);
        }
    });
    csv
}

/// Export transfer log as CSV: from,to,amount,timestamp
#[query]
fn export_transfers_csv() -> String {
    let mut csv = "from,to,amount,timestamp\n".to_string();
    TRANSFER_LOG.with(|log| {
        for e in log.borrow().iter() {
            csv += &format!("{},{},{},{}\n", e.from, e.to, e.amount, e.timestamp);
        }
    });
    csv
}
