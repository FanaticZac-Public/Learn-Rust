// Example 1 - The API of Mutex 

// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(5);

//     {
//         // lock() gets access and locks Mutex (for others)
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     } // lock goes out of scope and is dropped with deref trait

//     println!("m = {m:?}"); 
//     // m = Mutex { data: 6, poisoned: false, .. }
// }

// Example 2 - Shared Access to Mutex<T>

// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
//     // error[E0382]: borrow of moved value: `counter`
// }

// Rust is telling us that we can’t move the ownership of lock counter into multiple threads.


// Example 3 - Multiple Ownership with Multiple Threads

// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
//     // error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
// }
// //  The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`



// Example 4 - Atomic Reference Counting with Arc<T>
// use the atomic library from the API 

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // Result: 10
}