// using modules and crates
extern crate rand;
use rand::Rng;

// modules can be nested inside other modules (like directories)
mod greeeting {
    pub fn hello() -> String {
        return String::from("Hello, world!");
    }
}

fn main() {
    // 
    println!("{}", greeeting::hello());

    let random_number = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_number);
}