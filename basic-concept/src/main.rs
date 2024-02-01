use std::vec;

fn main(){
    //Variables and mutability
    let immutable_variable = 10;
    let mut mutable_variable = 5;

    println!("The immutable variable is: {}", immutable_variable);

    println!("The mutable variabel is: {}", mutable_variable);

    mutable_variable += immutable_variable;
    println!("After addition, the mutable variable is: {}", mutable_variable);

    //data types
    let string_example: String = String::from("Hello Rust!");
    let vector_example: Vec<i32>=vec![1,2,3];

    println!("String example: {}", string_example);
    println!("Vector example: {:?}", vector_example);

    //function and methods
    let result =  add_two_numbers(2,3);
    println!("2+3={}", result);

    if result > 4 {
        println!("The result is greater than 4");
    } else {
        println!("The result is not greate than 4");
    }

    match result {
        5 => println!("The result is exactly 5!"),
        _ => println!("The result is not 5"),
    }

    //call a method
    fn add_two_numbers(a: i32, b: i32) -> i32 {
        a + b //returning the sum of a and b
    }
}