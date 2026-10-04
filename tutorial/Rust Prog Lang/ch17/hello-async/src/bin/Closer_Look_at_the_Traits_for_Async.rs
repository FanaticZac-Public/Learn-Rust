// Example 1 - The Pin Type and the Unpin Trait

// fn main() {
//     trpl::block_on(async {
//         let tx_fut = async move {
//             // --snip--
//         };

//         let futures: Vec<Box<dyn Future<Output = ()>>> =
//             vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

//         trpl::join_all(futures).await;
//     });
// }
// error[E0277]: `dyn Future<Output = ()>` cannot be unpinned


// Example 2 - resolving the pin errors
use std::time::Duration;
use std::pin::{Pin, pin};

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        // let tx_fut = async move {
        //     // --snip--
        // };

        // let futures: Vec<Box<dyn Future<Output = ()>>> =
        //     vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        // -New- using pin! informs the compiler that a given type does not need to uphold any guarantees about whether the value in question can be safely moved.
        let tx1_fut = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
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
        });

        // -New-  Wrapping futures in pin ensures that the underlying data in the futures will not be moved
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];

        // This example now compiles and runs, and we could add or remove futures from the vector at runtime and join them all.    

        trpl::join_all(futures).await;
    });
}