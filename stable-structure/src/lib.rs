use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::StableBTreeMap;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, os::windows::thread};

// the state of the canister
#[derive(Serialize, Deserialize)]
struct State {
    #[serde(skip)]
    stable_btree: RefCell<StableBTreeMap<String, String>>, 
}

thread_local! {
    static STATE: Refcell<State> = RefCell::new(State {
        stable_btree: RefCell::new(StableBTreeMap::new())
    });
}

//retrives the value associated with the given key in the stable data if it exists
#[query]
fn get(key: String) -> Option<String> {
    STATE.with(|state| {
        state.borrow().stable_btree.borrow().get(&key).cloned()
    })
}

//Inserts entry info the map and returns the previous value of the key from stable data
#[update]
fn insert(key: String, value: String) -> Option<String> {
    STATE.with(|state| {
        state.borrow().stable_btree.borrow_mut().insert(key, value)
    })
}

impl Default for State {
    fn default() -> Self {
        State {
            stable_btree: RefCell::new(StableBTreeMap::new())
        }
    }
}  

fn main() {
    ic_cdk::setup();
}