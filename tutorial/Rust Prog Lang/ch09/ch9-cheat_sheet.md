# Chapter 09 - Error Handling - Cheat Sheet - Quick Reference

## Unrecoverable Errors with panic!

Rust groups errors into two major categories:

- **recoverable** - possible negative outcome of operation 
  - e.g. "File not found" 
  - probable action: report the problem to the user and retry
- **unrecoverable** - symptoms of bugs
  - e.g. "Index out of bounds"
  - probable action: Panic! Stop program.

Instead of exceptions (in most languages), **Rust** has:
-  **recoverable errors**
    - type Result<T, E> 
    - will return Result enum that returns Ok(T) or Err(E) that can be accounted for
        - compiler will complain if both are not accounted for
- **unrecoverable errors**
    - the panic! macro that stops execution 
    - Panics will print a failure message, unwind, clean up the stack, and quit.


### Unwinding the Stack or Aborting in Response to a Panic



## Recoverable Errors with Result



### Matching on Different Errors

### Propagating Errors

## To panic! or Not to panic!

### Examples, Prototype Code, and Tests

### When You Have More Information Than the Compiler

### Guidelines for Error Handling

### Custom Types for Validation

## Summary
