use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::api::caller;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
struct Account {
    balance: u64,
}

static mut ACCOUNTS: Option<HashMap<String, Account>> = None;

#[ic_cdk::init]
fn init() {
    unsafe {
        ACCOUNTS = Some(HashMap::new());
    }
    ic_cdk::println!("USDB_mint canister initialized");
}

#[ic_cdk::update]
fn mint(to: String, amount: u64) -> String {
    let caller_id = caller().to_text();
    ic_cdk::println!("Mint requested by: {}", caller_id);

    if amount == 0 {
        return "Mint amount must be greater than 0".to_string();
    }

    unsafe {
        let accounts = ACCOUNTS.as_mut().unwrap();
        let entry = accounts.entry(to.clone()).or_insert(Account { balance: 0 });
        entry.balance += amount;
        ic_cdk::println!("Minted {} USDB to {}", amount, to);
    }

    format!("Successfully minted {} USDB to {}", amount, to)
}

#[ic_cdk::update]
fn burn(from: String, amount: u64) -> String {
    unsafe {
        let accounts = ACCOUNTS.as_mut().unwrap();
        match accounts.get_mut(&from) {
            Some(account) => {
                if account.balance < amount {
                    return "Insufficient balance to burn".to_string();
                }
                account.balance -= amount;
                ic_cdk::println!("Burned {} USDB from {}", amount, from);
                format!("Successfully burned {} USDB from {}", amount, from)
            }
            None => "Account not found".to_string(),
        }
    }
}

#[ic_cdk::query]
fn balance_of(owner: String) -> u64 {
    unsafe {
        ACCOUNTS
            .as_ref()
            .unwrap()
            .get(&owner)
            .map(|acc| acc.balance)
            .unwrap_or(0)
    }
}
