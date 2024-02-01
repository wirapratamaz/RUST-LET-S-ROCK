use std::thread;

// this create a thread-local storage static variable named 'COUNTER'
thread_local! {
    static COUNTER: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
}

fn main() {
    // spawn a few threads and increment the counter in each thread
    let handles: Vec<_> = {0..3}.map(|_| {
        // each thread will have its own copy of 'COUNTER'
        thread::spawn(|| {
            // increment our thread's copy of the counter
            for _ in 0..10 {
                // access the thread-local storage
                COUNTER.with(|counter| {
                    // Get a mutan reference to the counter
                    let mut num = counter.borrow_mut();
                    // increment the value
                    *num += 1;
                    // print the value
                    println!("{}", *num);
                });
            }
        })
    }).collect();

    // wait for all threads to finish
    for handle in handles {
        // wait for the thread to finish. Returns a result.
        handle.join().unwrap();
    }
}
