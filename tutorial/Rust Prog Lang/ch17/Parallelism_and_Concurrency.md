# Parallelism and Concurrency

- Let's distinguish between parallelism and concurrency more precisely, because the differences will show up as we start working.
- Consider the different ways a team could split up work on a software project. You could assign a single member multiple tasks, assign each member one task, or use a mix of the two approaches.
- When an individual works on several different tasks before any of them is complete, this is concurrency.
  - One way to implement concurrency is similar to having two different projects checked out on your computer, and when you get bored or stuck on one project, you switch to the other.
- You’re just one person, so you can’t make progress on both tasks at the exact same time, but you can multitask, making progress on one at a time by switching between them

A concurrent workflow, switching between Task A and Task B

![A concurrent workflow, switching between Task A and Task B](https://doc.rust-lang.org/book/img/trpl17-01.svg)

When the team splits up a group of tasks by having each member take one task and work on it alone, this is parallelism.

![When the team splits up a group of tasks by having each member take one task and work on it alone, this is parallelism](https://doc.rust-lang.org/book/img/trpl17-02.svg)

- In both of these workflows, you might have to coordinate between different tasks.
- Maybe you thought the task assigned to one person was totally independent from everyone else’s work, but it actually requires another person on the team to finish their task first.
- Some of the work could be done in parallel, but some of it was actually serial: it could only happen in a series, one task after the other

A partially parallel workflow, where work happens on Task A and Task B independently until Task A3 is blocked on the results of Task B3.

![ A partially parallel workflow, where work happens on Task A and Task B independently until Task A3 is blocked on the results of Task B3.](https://doc.rust-lang.org/book/img/trpl17-03.svg)

- Likewise, you might realize that one of your own tasks depends on another of your tasks. 
    - Now your concurrent work has also become serial.
- Parallelism and concurrency can intersect with each other, too.   
    - If you learn that a colleague is stuck until you finish one of your tasks, you’ll probably focus all your efforts on that task to “unblock” your colleague.
-  You and your coworker are no longer able to work in parallel, and you’re also no longer able to work concurrently on your own tasks.
- On a machine with a single CPU core, the CPU can perform only one operation at a time, but it can still work concurrently. 
- Using tools such as threads, processes, and async, the computer can pause one activity and switch to others before eventually cycling back to that first activity again.
- On a machine with multiple CPU cores, it can also do work in parallel.
    - One core can be performing one task while another core performs a completely unrelated one, and those operations actually happen at the same time.
- Running async code in Rust usually happens concurrently.
- Depending on the hardware, the operating system, and the async runtime we are using (more on async runtimes shortly), that concurrency may also use parallelism under the hood.

