# Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams

Many operations we ask the computer to do can take a while to finish. It would be nice if we could do something else while we’re waiting for those long-running processes to complete. Your computer’s operating system can, and does, invisibly interrupt the export often enough to let you get other work done simultaneously.

Modern computers offer two techniques for working on more than one operation at a time: parallelism and concurrency. We’d like to be able to specify the operations a program should perform and points at which a function could pause and some other part of the program could run instead, without needing to specify up front exactly the order and manner in which each bit of code should run. 

Asynchronous programming is an abstraction that lets us express our code in terms of potential pausing points and eventual results that takes care of the details of coordination for us.

This chapter builds on Chapter 16’s use of threads for parallelism and concurrency by introducing an alternative approach to writing code: Rust’s futures, streams, and the async and await syntax that let us express how operations could be asynchronous, and the third-party crates that implement asynchronous runtimes: code that manages and coordinates the execution of asynchronous operations.

- A video export is an example of a CPU-bound or compute-bound operation. It’s limited by the computer’s potential data processing speed within the CPU or GPU, and how much of that speed it can dedicate to the operation.
- A video download is an example of an I/O-bound operation, because it’s limited by the speed of the computer’s input and output; it can only go as fast as the data can be sent across the network.

In both of these examples, the operating system’s invisible interrupts provide a form of concurrency. That concurrency happens only at the level of the entire program, though: the operating system interrupts one program to let other programs get work done. In many cases, because we understand our programs at a much more granular level than the operating system does, we can spot opportunities for concurrency that the operating system can’t see.

The operating system’s invisible interrupts provide a form of concurrency. That concurrency happens only at the level of the entire program, though: the operating system interrupts one program to let other programs get work done. In many cases, because we understand our programs at a much more granular level than the operating system does, we can spot opportunities for concurrency that the operating system can’t see.

If we’re building a tool to manage file downloads, we should be able to write our program so that starting one download won’t lock up the UI, and users should be able to start multiple downloads at the same time. Many operating system APIs block the program’s progress until the data they’re processing is completely ready. The term blocking is usually reserved for function calls that interact with files, but not the network, or other resources on the computer, because those are the cases where an individual program would benefit from the operation being non-blocking.

We could avoid blocking our main thread by spawning a dedicated thread to download each file. However, the overhead of the system resources used by those threads would eventually become a problem. It would be preferable if the call didn’t block in the first place, and instead we could define a number of tasks that we’d like our program to complete and allow the runtime to choose the best order and manner in which to run them.

That is exactly what Rust’s async (short for asynchronous) abstraction gives us. In this chapter, you’ll learn all about async as we cover the following topics:

- How to use Rust’s async and await syntax and execute asynchronous functions with a runtime
- How to use the async model to solve some of the same challenges we looked at in Chapter 16
- How multithreading and async provide complementary solutions that you can combine in many cases

[Parallelism and Concurrency - Background Refresher](./Parallelism_and_Concurrency.md)

Super Summary:
Concurrent: 1 thread multi-tasking.
    - Serial - the tasks are dependant on each other (orderly).
Parallel: 2 threads, each working on different tasks.

Note: This chapter using async/await - can be concurrence as opposed to threads - but the runtime library you are using could be using threads under the hood. So it's discussed as tasks.

Chapter 17 contains:

- [Futures and the Async Syntax](./Futures_and_the_Async_Syntax.md) 
- [Applying Concurrency with Async](./Applying_Concurrency_with_Async.md)
- [Working With Any Number of Futures](./Working_With_Any_Number_of_Futures.md)
- [Streams: Futures in Sequence](./Streams-Futures_in_Sequence.md)
- [A Closer Look at the Traits for Async](./Closer_Look_at_the_Traits_for_Async.md)
- [Putting It All Together: Futures, Tasks, and Threads](./Putting_It_Together-Futures_Tasks_Threads.md)


## Extra Ideas (Mine):
- Investigate trpl library since it's none standard.    
- Write a program that allows 2 clients to edit the same document with locks. (like a chess game - click the clock to release the lock)


## Post Summary

This isn’t the last you’ll see of concurrency in this book. The project in Chapter 21 will apply these concepts in a more realistic situation than the simpler examples discussed here and compare problem-solving with threading versus tasks and futures more directly.

No matter which of these approaches you choose, Rust gives you the tools you need to write safe, fast, concurrent code—whether for a high-throughput web server or an embedded operating system.
