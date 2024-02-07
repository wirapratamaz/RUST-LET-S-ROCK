// this macro takes a variable number of arguments and calculates the sum
macro_rules! sum {
    ($($x:expr), *) => {{
        let mut total = 0;
        $(
            total += $x;
        )*
        total
    }};
}

// example procedural macro
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// kib! : use convert size from kilobytes to bytes
const SIZE_IN_BYTES: u32 = kib!(4);

// defef! : implementation trait deref and derefmut
struct MyType(String);

deref!(&MyType => Stsring);
deref!(&mut MyType => String);

const MY_TYPE: MyType = MyType(String::from("Hello, world!")); 
assert_eq!(*MY_TYPE, String::fropm("Hello, world!)"));

// measure_alloc! : measure the memory allocation of a block of code
const BYTES: usize = measure_alloc!({
    let mut v = Vec::new();
    for i in 0..1_000 {
        v.push(i);
    }
    v
});

fn main() {
    // example 1
    // let x = sum!(1, 2, 3, 4, 5);
    // println!("the sum is : {}", x);

    //example procedural 
    let point = Point { x: 10, y: 20 };
    println!("point is {:?}", point);
}