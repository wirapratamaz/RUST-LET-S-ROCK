fn main() {
    // Ownership
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2 and can no longer be used
    println!("{}, world!", s2);

    // Borrowing
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // s3 is borrowed
    println!("The length of '{}' is {}.", s3, len);

    // Mutable borrow references
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // Lifetime
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Borrowing
fn calculate_length(s: &String -> usize) {
    s.len()
}
```

// Mutable borrow references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
