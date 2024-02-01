// However, this won't work because you can only use .await inside an async function.
// fn main(){
//     // let future = hello();
//     // let greeeting =  future.await;
//     // println!("{}", greeeting);
//     
// }

// So, we need to make main async too.
#[tokio::main] // tokio is a runtime for async Rust
async fn main(){
    let greeeting = hello().await; // await is like .then() in JS
    println!("{}", greeeting); // Hello, world!
}

//async func
async fn hello() -> String {
    String::from("Hello, world!")
}
