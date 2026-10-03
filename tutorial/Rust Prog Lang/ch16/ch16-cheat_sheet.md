# Fearless Concurrency

Handling concurrent programming safely and efficiently is another of Rust’s major goals.

Concurrent programming, in which different parts of a program execute independently, and parallel programming, in which different parts of a program execute at the same time, has historically been difficult and error-prone.

Ensuring memory safety and preventing concurrency problems are the same problem - and ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems!

By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.

As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production. 

Note: Mentally substitute concurrent and/or parallel whenever we use concurrent.

Many languages are dogmatic about the solutions they offer for handling concurrent problems. For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads. Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages because a higher-level language promises benefits from giving up some control to gain abstractions. However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:

- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library

Chapter 14 contains:

- [Using Threads to Run Code Simultaneously](./Threads.md) - Done (pretty much perfect)
- [Transfer Data Between Threads with Message Passing](./Threads_Data_Passing.md) - Done (pretty much perfect)
- [Shared-State Concurrency](./Shared-State_Concurrency.md)  - Done (pretty much perfect)
- [Extensible Concurrency with Send and Sync](./Send_and_Sync-Extensible_Concurrency.md)  - Done (pretty much perfect)

## Exercise options:
- Try creating a Rust program that has a deadlock with Mutex<T>

## Post Summary

This isn’t the last you’ll see of concurrency in this book: The next chapter focuses on async programming, and the project in Chapter 21 will use the concepts in this chapter in a more realistic situation than the smaller examples discussed here.

As mentioned earlier, because very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates. These evolve more quickly than the standard library, so be sure to search online for the current, state-of-the-art crates to use in multithreaded situations.

The Rust standard library provides channels for message passing and smart pointer types, such as Mutex<T> and Arc<T>, that are safe to use in concurrent contexts. The type system and the borrow checker ensure that the code using these solutions won’t end up with data races or invalid references. Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages. Concurrent programming is no longer a concept to be afraid of.