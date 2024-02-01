use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;

#[derive(CandidType, Deserialize, Clone, Default)]
struct Counter {
    count: u64,
}

#[update]
#[candid::candid_method(update)]
fn increment() {
    let counter = ic_cdk::storage::get_mut::<Counter>().unwrap();
    counter.count += 1;
}

#[query]
#[candid::candid_method(query)]
fn get() -> u64 {
    let counter = ic_cdk::storage::get::<Counter>().unwrap();
    counter.count
}

fn main() {
    // initialize the canister
    ic_cdk::setup();
}