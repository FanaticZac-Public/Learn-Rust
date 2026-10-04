// Example 1 - Creating a New Task with spawn_task

// use std::time::Duration;

// fn main() {
//     trpl::block_on(async {
//         trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//     });
// }

// Example 2 - awaiting result for handle spawn
// use std::time::Duration;
// fn main() {
//     trpl::block_on(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         handle.await.unwrap();
//     });
// }

// Example 3 - awaiting result on both tasks

// use std::time::Duration;

// fn main() {
//     trpl::block_on(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         // Awaits both to complete now.
//         trpl::join(fut1, fut2).await;
//     });
// }


// Example 4 - Sending Data Between Two Tasks Using Message Passing 

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let val = String::from("hi");
//         tx.send(val).unwrap();

//         let received = rx.recv().await.unwrap();
//         println!("received '{received}'");
//     });
// }
// Notice two things about this example. First, the message will arrive right away. Second, although we use a future here, there’s no concurrency yet. Everything in the listing happens in sequence, just as it would if there were no futures involved.

// Example 5 - Send multiple messages

// use std::time::Duration;
// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("future"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         while let Some(value) = rx.recv().await {
//             println!("received '{value}'");
//         }
//     });
// }  
// // The code now successfully sends and receives all of the messages. Unfortunately, there are still a couple of problems. For one thing, the messages do not arrive at half-second intervals. They arrive all at once, 2 seconds (2,000 milliseconds) after we start the program. For another, this program also never exits! 
// // - There’s only one async block, so everything in it runs linearly. 
// // - There’s still no concurrency.
// // - All the tx.send calls happen, interspersed with all of the trpl::sleep calls and their associated await points. 
// // - Only then does the while let loop get to go through any of the await points on the recv calls.


// Example 6 - Proper async messages
// use std::time::Duration;

// fn main() {
//     trpl::block_on(async {
//         let (tx, mut rx) = trpl::channel();

//         let tx_fut = async {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         trpl::join(tx_fut, rx_fut).await;
//     });
// }  

// Example 7 - Moving tx into async block with move keyword to shutdown that arm after messages sent.
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}  



