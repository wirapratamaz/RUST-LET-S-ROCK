use std::rc::Rc; // Rc is a reference counting pointer
use std::cell::RefCell; // RefCell is a mutable memory location with dynamically checked borrow rules

#[derive(Debug)]
// struct node for tree data structure with value and children node
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main(){
    // create a leaf node
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    // create a branch node
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // print the leaf and branch
    println!("leaf: {:?}", leaf);
    println!("branch: {:?}", branch);
}