# Object-Oriented Programming Features

Object-oriented programming (OOP) is a way of modeling programs.

Objects as a programmatic concept were introduced in the programming language Simula in the 1960s.

Those objects influenced Alan Kay’s programming architecture in which objects pass messages to each other. To describe this architecture, he coined the term object-oriented programming in 1967.

Many competing definitions describe what OOP is, and by some of these definitions Rust is object oriented but by others it is not.

In this chapter, we’ll explore certain characteristics that are commonly considered object oriented and how those characteristics translate to idiomatic Rust. 

We’ll then show you how to implement an object-oriented design pattern in Rust and discuss the trade-offs of doing so versus implementing a solution using some of Rust’s strengths instead.


Chapter 18 contains:

- [Characteristics of Object-Oriented Languages](./Characteristics_of_Object-Oriented_Languages.md) 
- [Using Trait Objects to Abstract over Shared Behavior](./Using%20Trait_Objects_to_Abstract_over_Shared_Behavior.md)
- [Implementing an Object-Oriented Design Pattern](./Implementing_an_Object-Oriented_Design_Pattern.md)

## Post Summary

- Regardless of whether you think Rust is an object-oriented language after reading this chapter, you now know that you can use trait objects to get some object-oriented features in Rust. 
- Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance. 
- You can use this flexibility to implement object-oriented patterns that can help your code’s maintainability. 
- Rust also has other features, like ownership, that object-oriented languages don’t have. 
- An object-oriented pattern won’t always be the best way to take advantage of Rust’s strengths, but it is an available option.