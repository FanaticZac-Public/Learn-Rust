# Using Threads to Run Code Simultaneously
## Super Summarize

```rust
//-------------- Creating a New Thread with spawn --------------
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
} 
// This code will stop the spawned thread prematurely due to the main thread ending, and there is no guarantee on the order in which threads run.


//-------------- Waiting for All Threads to Finish --------------
use std::thread;
use std::time::Duration;

fn main() {
    //  saving the return value of thread::spawn in a variable
    // The return type of thread::spawn is JoinHandle<T>
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // A JoinHandle<T> is an owned value that, when we call the join method on it, will wait for its thread to finish.
    handle.join().unwrap();
}
// If we move handle.join().unwrap(); between the loops, it will wait for all spawned threads to complete before moving past this line.


//-------------- Using move Closures with Threads --------------
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
// error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
// - Rust can’t tell how long the spawned thread will run, so it doesn’t know whether the reference to v will always be valid.

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
// Using the 'move' keyword - we take ownership of v (invalidating old v variable)
```

## Using Threads to Run Code Simultaneously

In most current operating systems:
- program’s code is run in a process (OS manages processes)
- Within a program, you can also have independent parts that run simultaneously, called threads.

For example, a web server could have multiple threads so that it can respond to more than one request at the same time.

Splitting the computation in your program into multiple threads to run multiple tasks at the same time can improve performance, but it also adds complexity. 

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:

- Race conditions, in which threads are accessing data or resources in an inconsistent order
- Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing
- Bugs that only happen in certain situations and are hard to reproduce and fix reliably

Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought and requires a code structure that is different from that in programs running in a single thread.

- Programming languages implement threads in a few different ways, and many operating systems provide an API the programming language can call for creating new threads.
- The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.
- There are crates that implement other models of threading that make different trade-offs to the 1:1 model. (Rust’s async system, which we will see in the next chapter, provides another approach to concurrency as well.)

## Creating a New Thread with spawn

To create a new thread, we call the thread::spawn function and pass it a closure (we talked about closures in Chapter 13) containing the code we want to run in the new thread.

The example in Listing 16-1 prints some text from a main thread and other text from a new thread.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
```

Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.

The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run. The threads will probably take turns, but that isn’t guaranteed: It depends on how your operating system schedules the threads.

In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code. And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down.

## Waiting for All Threads to Finish

The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending, but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!

We can fix the problem of the spawned thread not running or of it ending prematurely by saving the return value of thread::spawn in a variable.

The return type of thread::spawn is JoinHandle<T>.

A JoinHandle<T> is an owned value that, when we call the join method on it, will wait for its thread to finish.

Here shows how to use the JoinHandle<T> of the thread we created in Listing 16-1 and how to call join to make sure the spawned thread finishes before main exits.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 1 from the spawned thread!
// hi number 3 from the main thread!
// hi number 2 from the spawned thread!
// hi number 4 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
```

- Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. 
- Blocking a thread means that thread is prevented from performing work or exiting. 

The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.

But let’s see what happens when we instead move handle.join() before the for loop in main, like this:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore, as shown here:

```
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

## Using move Closures with Threads

We’ll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another.

To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs. 

An attempt to create a vector in the main thread and use it in the spawned thread. However, this won’t work yet, as you’ll see in a moment:

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
// error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
```

The closure uses v, so it will capture v and make it part of the closure’s environment. Because thread::spawn runs this closure in a new thread, we should be able to access v inside that new thread.

Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know whether the reference to v will always be valid.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
// using move works - invalidates the old variable
```