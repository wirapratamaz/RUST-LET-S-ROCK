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

fn main() {
    // example 1
    // let x = sum!(1, 2, 3, 4, 5);
    // println!("the sum is : {}", x);

    //example procedural 
    let point = Point { x: 10, y: 20 };
    println!("point is {:?}", point);
}