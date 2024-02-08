// main.rs
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};
use ic_stable_structures::{writer::Writer, Memory as _, StableBTreeMap};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
mod memory;
use memory::Memory;

// The state of the canister.
#[derive(Serialize, Deserialize)]
struct State {
    data_on_the_heap: Vec<u8>,
    #[serde(skip, default = "init_stable_data")]
    stable_data: StableBTreeMap<u128, u128, Memory>,
}

// The state of the canister is stored in a thread-local variable.
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

// The following functions are the public entry points to the canister.
#[query]
fn stable_get(key: u128) -> Option<u128> {
    STATE.with(|s| s.borrow().stable_data.get(&key))
}

#[update]
fn stable_insert(key: u128, value: u128) -> Option<u128> {
    STATE
        .with(|s| s.borrow_mut().stable_data.insert(key, value))
}

#[update]
fn set_heap_data(data: Vec<u8>) {
    STATE.with(|s| s.borrow_mut().data_on_the_heap = data);
}

#[query]
fn get_heap_data() -> Vec<u8> {
    STATE.with(|s| s.borrow().data_on_the_heap.clone())
}

// The following functions are used to upgrade the canister.
#[pre_upgrade]
fn pre_upgrade() {
    let mut state_bytes = vec![];
    STATE.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    let len = state_bytes.len() as u32;
    let mut memory = memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

// The following functions are used to initialize the canister after an upgrade.
#[post_upgrade]
fn post_upgrade() {
    let memory = memory::get_upgrades_memory();

    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| {
        *s.borrow_mut() = state
    });
}

fn init_stable_data() -> StableBTreeMap<u128, u128, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

impl Default for State {
    fn default() -> Self {
        Self {
            data_on_the_heap: vec![],
            stable_data: init_stable_data(),
        }
    }
}