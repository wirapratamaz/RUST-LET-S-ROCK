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

// define Profile struct
#[derive(CandidType, Deserialize)]
struct Greeting {
    name: String,
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

// define main function
fn main() {}
```