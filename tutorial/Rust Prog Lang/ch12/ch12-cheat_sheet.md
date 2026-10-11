

# Chapter 12 - An I/O Project: Building a Command Line Program - Cheat Sheet - Quick Reference

Topics Covered:

- Command Line Arguments
- File operations
- Error handling
- Test organization
  - Unit Tests
  - Integration tests
- Test Driven Development
- Redirect output to stdout vs stderr

Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools

This chapter recreates grep command
- grep: (globally search a regular expression and print).

One Rust community member, Andrew Gallant, has already created a fully featured, very fast version of grep, called ripgrep

## This chapter contains sub-sections below: 
- [Accepting Command Line Arguments](./Accepting_Command_Line_Arguments.md)
- [Reading a File](./Reading_a_File.md)
- [Refactoring to Improve Modularity and Error Handling](./Improve_Modularity_and_Error_Handling.md)
- [Adding Functionality with Test Driven Development](./Adding_Test_Driven_Development.md)
- [Working with Environment Variables](./Working_with_Environment_Variables.md)
- [Redirecting Errors to Standard Error](./Redirecting_Errors_to_Standard_Error.md)


## Post Summary

This chapter recapped some of the major concepts you’ve learned so far and covered how to perform common I/O operations in Rust. By using command line arguments, files, environment variables, and the eprintln! macro for printing errors, you’re now prepared to write command line applications. Combined with the concepts in previous chapters, your code will be well organized, store data effectively in the appropriate data structures, handle errors nicely, and be well tested.

## Other / Related Links

- [env (Module)](https://doc.rust-lang.org/std/env/index.html)
  - [Args (Struct)](https://doc.rust-lang.org/std/env/fn.args.html)
- [Iterator (Trait)](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  - [collect (method)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [fs (Module)](https://doc.rust-lang.org/std/fs/index.html)
  - [read_to_string (Function)](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
- [process (Module)](https://doc.rust-lang.org/std/process/index.html)
  - [exit (Function)](https://doc.rust-lang.org/std/process/fn.exit.html)
- [Result (Enum)](https://doc.rust-lang.org/std/result/enum.Result.html)
  - [unwrap_or_else (Function)](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else)
- [error (Module)](https://doc.rust-lang.org/std/error/index.html)
  - [Error (Trait)](https://doc.rust-lang.org/std/error/trait.Error.html)

## New Keywords Introduced

- dyn: Dynamic dispatch to a trait object.
