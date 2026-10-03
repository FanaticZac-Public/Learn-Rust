

// Example 1 - just running one extra thread

// use std::thread;
// use std::time::Duration;

// fn main() {
//     // 1 additional thread
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1)); 
//         }
//     });

//     // main thread
//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(10)); // changed to 10
//     }
// }

// Example 2 - Waiting for All Threads to Finish

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }


// Example 3 - Waiting for All Threads to Finish - moved handle earlier in code

// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {i} from the spawned thread!");
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();
//     // This results in spawned thread completing it's work before moving past this point.

//     for i in 1..5 {
//         println!("hi number {i} from the main thread!");
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// Example 4 - Using move Closures with Threads

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {v:?}");
//     });

//     handle.join().unwrap();
// }

// error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function

// FIX - USING MOVE

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
// WORKS NOW