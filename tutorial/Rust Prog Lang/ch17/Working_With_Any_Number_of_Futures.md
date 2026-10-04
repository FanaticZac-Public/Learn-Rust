# Yielding Control to the Runtime

## Super Summary

```rust
//-------------- Yielding Control to the Runtime --------------
// Let’s simulate a long-running operation to illustrate the starvation problem, then explore how to solve it. 
use std::time::Duration;
use std::thread;

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    trpl::block_on(async {
        // Example 1 slow and sequential

        // let a = async {
        //     println!("'a' started.");
        //     slow("a", 30);
        //     slow("a", 10);
        //     slow("a", 20);
        //     trpl::sleep(Duration::from_millis(50)).await;
        //     println!("'a' finished.");
        // };

        // let b = async {
        //     println!("'b' started.");
        //     slow("b", 75);
        //     slow("b", 10);
        //     slow("b", 15);
        //     slow("b", 350);
        //     trpl::sleep(Duration::from_millis(50)).await;
        //     println!("'b' finished.");
        // };

        // Example 2 - interweaved and asynchronous
        // let one_ms = Duration::from_millis(1);

        // let a = async {
        //     println!("'a' started.");
        //     slow("a", 30);
        //     trpl::sleep(one_ms).await;
        //     slow("a", 10);
        //     trpl::sleep(one_ms).await;
        //     slow("a", 20);
        //     trpl::sleep(one_ms).await;
        //     println!("'a' finished.");
        // };

        // let b = async {
        //     println!("'b' started.");
        //     slow("b", 75);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 10);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 15);
        //     trpl::sleep(one_ms).await;
        //     slow("b", 350);
        //     trpl::sleep(one_ms).await;
        //     println!("'b' finished.");
        // };

        // Example 3 - asynchronous - yielding instead of sleeping (faster)

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };


        trpl::select(a, b).await;
    });
}


//-------------- Building Our Own Async Abstractions --------------
// The implementation of trpl::select is not fair: it always polls arguments in the order in which they are passed 
// we pass future_to_try to select first so it gets a chance to complete even if max_time is a very short duration.

use std::time::Duration;
use std::thread;
use trpl::Either;


async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
// Because futures compose with other futures, you can build really powerful tools using smaller async building blocks. 
// For example, you can use this same approach to combine timeouts with retries, and in turn use those with operations such as network calls
```

## Yielding Control to the Runtime

Recall from the “Our First Async Program” section that 
- at each await point, Rust gives a runtime a chance to pause the task and switch to another one if the future being awaited isn’t ready. 

The inverse is also true: 

- Rust only pauses async blocks and hands control back to a runtime at an await point. Everything between await points is synchronous.

- That means if you do a bunch of work in an async block without an await point, that future will block any other futures from making progress. 
- You may sometimes hear this referred to as one future starving other futures.
- In some cases, that may not be a big deal. However, if you are doing some kind of expensive setup or long-running work, or if you have a future that will keep doing some particular task indefinitely, you’ll need to think about when and where to hand control back to the runtime.

Let’s simulate a long-running operation to illustrate the starvation problem, then explore how to solve it. 

- This code uses std::thread::sleep instead of trpl::sleep so that calling slow will block the current thread for some number of milliseconds. 
- We can use slow to stand in for real-world operations that are both long-running and blocking.
- we use slow to emulate doing this kind of CPU-bound work in a pair of futures.

```rust
use std::time::Duration;
use std::thread;

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}
// 'a' started.
// 'a' ran for 30ms
// 'a' ran for 10ms
// 'a' ran for 20ms
// 'b' started.
// 'b' ran for 75ms
// 'b' ran for 10ms
// 'b' ran for 15ms
// 'b' ran for 350ms
// 'a' finished.
```

- We used trpl::select to race futures fetching two URLs, select still finishes as soon as a is done. 
- There’s no interleaving between the calls to slow in the two futures, though. 
- The a future does all of its work until the trpl::sleep call is awaited, then the b future does all of its work until its own trpl::sleep call is awaited, and finally the a future completes. 
- To allow both futures to make progress between their slow tasks, we need await points so we can hand control back to the runtime. That means we need something we can await!

- We can already see this kind of handoff happening: if we removed the trpl::sleep at the end of the a future, it would complete without the b future running at all. 
Let’s try using the trpl::sleep function as a starting point for letting operations switch off making progress.

```rust
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };
```
We’ve added trpl::sleep calls with await points between each call to slow. Now the two futures’ work is interleaved:

```
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

- The a future still runs for a bit before handing off control to b, because it calls slow before ever calling trpl::sleep, but after that the futures swap back and forth each time one of them hits an await point. 
- In this case, we have done that after every call to slow, but we could break up the work in whatever way makes the most sense to us.

- We don’t really want to sleep here, though: we want to make progress as fast as we can. 
- We just need to hand back control to the runtime. 
- We can do that directly, using the trpl::yield_now function.

```rust
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };
```

- This code is both clearer about the actual intent and can be significantly faster than using sleep, because timers such as the one used by sleep often have limits on how granular they can be. - - The version of sleep we are using, for example, will always sleep for at least a millisecond, even if we pass it a Duration of one nanosecond. Again, modern computers are fast: they can do a lot in one millisecond!

- This means that async can be useful even for compute-bound tasks, depending on what else your program is doing, because it provides a useful tool for structuring the relationships between different parts of the program (but at a cost of the overhead of the async state machine). 
- This is a form of cooperative multitasking, where each future has the power to determine when it hands over control via await points. Each future therefore also has the responsibility to avoid blocking for too long. 
- In some Rust-based embedded operating systems, this is the only kind of multitasking!

In real-world code, you won’t usually be alternating function calls with await points on every single line, of course. While yielding control in this way is relatively inexpensive, it’s not free. In many cases, trying to break up a compute-bound task might make it significantly slower, so sometimes it’s better for overall performance to let an operation block briefly. Always measure to see what your code’s actual performance bottlenecks are. The underlying dynamic is important to keep in mind, though, if you are seeing a lot of work happening in serial that you expected to happen concurrently!

## Building Our Own Async Abstractions

- We can also compose futures together to create new patterns. For example, we can build a timeout function with async building blocks we already have. 
- When we’re done, the result will be another building block we could use to create still more async abstractions.

```rust
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
```

- Let’s implement this! To begin, let’s think about the API for timeout:

- It needs to be an async function itself so we can await it.
- Its first parameter should be a future to run. We can make it generic to allow it to work with any future.
- Its second parameter will be the maximum time to wait. If we use a Duration, that will make it easy to pass along to trpl::sleep.
- It should return a Result. If the future completes successfully, the Result will be Ok with the value produced by the future. 
    - If the timeout elapses first, the Result will be Err with the duration that the timeout waited for.

```rust
async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    // Here is where our implementation will go!
}
```

- That satisfies our goals for the types. 
- Now let’s think about the behavior we need: we want to race the future passed in against the duration. 
- We can use trpl::sleep to make a timer future from the duration, and use trpl::select to run that timer with the future the caller passes in.

we implement timeout by matching on the result of awaiting trpl::select.

```rust
use trpl::Either;

// --snip--

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
```

- The implementation of trpl::select is not fair: it always polls arguments in the order in which they are passed (other select implementations will randomly choose which argument to poll first). 
- Thus, we pass future_to_try to select first so it gets a chance to complete even if max_time is a very short duration. 
- If future_to_try finishes first, select will return Left with the output from future_to_try. 
- If timer finishes first, select will return Right with the timer’s output of ().

If the future_to_try succeeds and we get a Left(output), we return Ok(output). If the sleep timer elapses instead and we get a Right(()), we ignore the () with _ and return Err(max_time) instead.

With that, we have a working timeout built out of two other async helpers. If we run our code, it will print the failure mode after the timeout:

```
Failed after 2 seconds
```

- Because futures compose with other futures, you can build really powerful tools using smaller async building blocks. 
- For example, you can use this same approach to combine timeouts with retries, and in turn use those with operations such as network calls.

In practice, you’ll usually work directly with async and await, and secondarily with functions such as select and macros such as the join! macro to control how the outermost futures are executed.