# Chapter 09 - Error Handling - Cheat Sheet

Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. 

Rust groups errors into two major categories: **recoverable** and **unrecoverable** errors.

- For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation.
- Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions.

Rust doesn’t have exceptions.

- Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.
  - Panics will print a failure message, unwind, clean up the stack, and quit.

This chapter contains: 
- [Unrecoverable Errors with panic!](./Unrecoverable_Errors_Panic.md)
- [Recoverable Errors with Result](./Recoverable_Errors_With_Result.md)
- [To panic! or Not to panic!](./Panic_Or_Not.md)


## Post Summary

Rust’s error-handling features are designed to help you write more robust code. The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well. Using panic! and Result in the appropriate situations will make your code more reliable in the face of inevitable problems.

