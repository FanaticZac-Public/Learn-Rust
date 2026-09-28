<style>
@import url("../notes.css");
</style>

# Chapter 13 - Functional Language Features: Iterators and Closures - Cheat Sheet - Quick Reference

Rust’s design is significantly influenced by _functional programming_.

- Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.
- Other Rust features, such as pattern matching and enums, are also influenced by the functional style.

Covers:

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use closures and iterators to improve the I/O project in Chapter 12
- The performance of closures and iterators (spoiler alert: They’re faster than you might think!)

Mastering closures and iterators is an important part of writing fast, idiomatic, Rust code

<div class="zac-note">
I'm now splitting up the cheat sheets into separate subsections so i can create links by topic on another page.
</div>


Chapter 13 discusses:
- [Closures](./Closures.md)
- [Iterators](./Iterators.md)
- [Improving Our I/O Project](./Improving-IO-Project.md)
- [Performance(Loops vs. Iterators)](./Performance-Loops_vs_Iterators.md)



