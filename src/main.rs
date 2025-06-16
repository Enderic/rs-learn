/*
    Because we can't use Rc<T>, we use Arc<T> which is safe for concurrent/parallel situations
        - the A stands for atomic; just know as of now it's just primitive types safe to use across threads
        - The reason all types aren't thread safe is bc it comes with a performance penalty
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main()  {
    // Arc<T> & Rc<T> have same API 
    // In simple cases like this, we can use types in std::sync::atomic 
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Notice how we could get a mutable refernce to counter despite counter itself not being mutable
            // Mutex<T> provides interior mutability (The Cell family does this)
            // Mutex<T> can create deadlocks so be careful
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Result: {}", *counter.lock().unwrap());
}