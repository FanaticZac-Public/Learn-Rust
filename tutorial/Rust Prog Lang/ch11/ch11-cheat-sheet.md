# Chapter 11 - Writing Automated Tests

Original Chapter: [Writing Automatic Tests - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-00-testing.html)

Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove. Rust’s type system shoulders a huge part of this burden, but the type system cannot catch everything. As such, Rust includes support for writing automated software tests.

Rust does all the type checking and borrow checking that you’ve learned so far to ensure we aren’t passing a String value or an invalid reference to a function. But Rust can’t check that this function will do precisely what we intend in a given circumstance logically.

We can write tests that assert that when we pass values to a function that we return the correct value. We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill: We will discuss the mechanics of Rust’s testing facilities. We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options provided for running your tests, and how to organize tests into unit tests and integration tests.

## This chapter contains sub-sections below: 
- [How to Write Tests](./Tests-How_to_Write.md)
- [Controlling How Tests Are Run](./Tests-Controlling_How_They_Run.md)
- [Test Organization](./Tests-Organization.md)

## Post Summary

Rust’s testing features provide a way to specify how code should function to ensure that it continues to work as you expect, even as you make changes. 

Unit tests exercise different parts of a library separately and can test private implementation details. 

Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it. 

Even though Rust’s type system and ownership rules help prevent some kinds of bugs, tests are still important to reduce logic bugs having to do with how your code is expected to behave.