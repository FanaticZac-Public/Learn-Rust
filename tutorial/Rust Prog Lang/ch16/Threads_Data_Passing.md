# Transfer Data Between Threads with Message Passing

## Super Summarize

```rust
//-------------- Transfer Data Between Threads with Message Passing --------------
// Basic channels communication setup - spawn to main thread.
use std::sync::mpsc; // multiple producer, single consumer.
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // Got: hi
}
// The try_recv(), rather than recv(), method doesn’t block, and will instead return a Result<T, E> immediately, useful if this thread has other work to do while waiting for messages: We could write a loop that calls try_recv every so often


//-------------- Transferring Ownership Through Channels --------------
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}"); // We try to print value after having sent it (moving it)
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // error[E0382]: borrow of moved value: `val`
}


//-------------- Sending Multiple Values --------------
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    // Got: hi
    // Got: from
    // Got: the
    // Got: thread
}


//-------------- Creating Multiple Producers --------------
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone(); // cloning the transmitter,
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap(); // Sender 1 - Copy 
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap(); // Sender 0 
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {received}");
}

// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: for
// Got: the
// Got: thread
// Got: you
}

```

## Transfer Data Between Threads with Message Passing

One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.

Here’s the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”

To accomplish message-sending concurrency, Rust’s standard library provides an implementation of channels. A channel is a general programming concept by which data is sent from one thread to another.

A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put the rubber duck into the river, and the receiver half is where the rubber duck ends up downstream. One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages. A channel is said to be closed if either the transmitter or receiver half is dropped.

Here, we’ll work up to a program that has one thread to generate values and send them down a channel, and another thread that will receive the values and print them out. We’ll be sending simple values between threads using a channel to illustrate the feature. Once you’re familiar with the technique, you could use channels for any threads that need to communicate with each other, such as a chat system or a system where many threads perform parts of a calculation and send the parts to one thread that aggregates the results.

- We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
- In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values. 
- Imagine multiple streams flowing together into one big river: Everything sent down any of the streams will end up in one river at the end. 
- We’ll start with a single producer for now, but we’ll add multiple producers when we get this example working.
- The mpsc::channel function returns a tuple, the first element of which is the sending end—the transmitter—and the second element of which is the receiving end—the receiver. 
- The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver, respectively, so we name our variables as such to indicate each end. 
- We’re using a let statement with a pattern that destructures the tuples
- Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so that the spawned thread owns tx. 
- The spawned thread needs to own the transmitter to be able to send messages through the channel.
- The transmitter has a send method that takes the value we want to send. 
- The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.
-  In this example, we’re calling unwrap to panic in case of an error. But in a real application, we would handle it properly
- we’ll get the value from the receiver in the main thread. This is like retrieving the rubber duck from the water at the end of the river or receiving a chat message.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```
- The receiver has two useful methods: recv and try_recv. 
- We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
- Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.
- The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. 
- Using try_recv is useful if this thread has other work to do while waiting for messages: 
- We could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.

## Transferring Ownership Through Channels

- The ownership rules play a vital role in message sending because they help you write safe, concurrent code. 
- Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs.
- Let’s do an experiment to show how channels and ownership work together to prevent problems: We’ll try to use a val value in the spawned thread after we’ve sent it down the channel. 

Try compiling the code in Listing 16-9 to see why this code isn’t allowed.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // error[E0382]: borrow of moved value: `val`
}
```

- Here, we try to print val after we’ve sent it down the channel via tx.send.
- Allowing this would be a bad idea: Once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again. 
- Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data.
- Our concurrency mistake has caused a compile-time error. 
- The send function takes ownership of its parameter, and when the value is moved the receiver takes ownership of it.
- This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.

## Sending Multiple Values

we’ve made some modifications that will prove the code in Listing 16-8 is running concurrently: The spawned thread will now send multiple messages and pause for a second between each message.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    // Got: hi
    // Got: from
    // Got: the
    // Got: thread
}
```

Because we don’t have any code that pauses or delays in the for loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.

## Creating Multiple Producers

Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer. 
Let’s put mpsc to use and expand the code to create multiple threads that all send values to the same receiver. 
- We can do so by cloning the transmitter,

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

let (tx, rx) = mpsc::channel();

let tx1 = tx.clone(); // cloning the transmitter,
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap(); // Sender 1 - Copy 
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap(); // Sender 0 
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {received}");
}

// Got: hi
// Got: more
// Got: from
// Got: messages
// Got: for
// Got: the
// Got: thread
// Got: you
}
```

- This time, before we create the first spawned thread, we call clone on the transmitter. 
- This will give us a new transmitter we can pass to the first spawned thread. 
- We pass the original transmitter to a second spawned thread. 
- This gives us two threads, each sending different messages to the one receiver.
- You might see the values in another order, depending on your system. This is what makes concurrency interesting as well as difficult. If you experiment with thread::sleep, giving it various values in the different threads, each run will be more nondeterministic and create different output each time.