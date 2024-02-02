// define candid interface
// type Profile = record {
//     id: nat;
//     name: text;
//     age: nat;
// };

// service : {
//     getProfile : () -> (Profile);
// }

// import ic_cdk to use candid macro
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
use std::collections::HashMap;

// define Profile struct
#[derive(CandidType, Deserialize)]
struct Greeting {
    name: String,
}

// define State struct
#[derive(Default)]
struct State {
    profiles: HashMap<String, String>,
}

// initialize the state of the canister
#[init]
fn init(){
    let state = State {
        greetings: HashMap::new(),
    };
    ic_cdk::storage::put(state);
}

// Update functions can modify the state of the canister
#[update]
// define getProfile function
fn greet(g: Greeting) -> String {
    format!("Hello, {}!", g.name)
}

// Query functions cannot modify the state of the canister. They are read-only and are used to retrieve information from the canister
#[query]
// define hello_world query function
fn hello_world() -> String {
    "Hello, world!".to_string()
}

// can modify the state of the canister
fn set_greeting(g: Greeting, greeting: String){
    let state = ic_sdk::storage::get_mut::<State>();
    state.greetings.insert(g.name, greeting);
}

// cannot modify the state of the canister
fn get_greeting(g:Greeting) -> Option<String>{
    let state = ic_sdk::storage::get::<State>();
    state.greetings.get(&g.name).cloned()
}

// define main function
fn main() {}
```