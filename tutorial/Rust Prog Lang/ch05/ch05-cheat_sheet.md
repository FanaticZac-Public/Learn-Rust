# Chapter 05 - Using Structs to Structure Related Data

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group, like an object’s data attributes. 

Topics: 
- Compare and contrasting tuples with structs
- How to define and instantiate structs. 
- Defining methods and other associated functions to specify behavior associated with a struct type. 

This chapter contains: 
- [Defining and Instantiating Structs](./Defining_&_Initializing_Structs.md)
- [An Example Program Using Structs](./Example_Program_Structs.md)
- [Methods](./Methods.md)

## Summary

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

But structs aren’t the only way you can create custom types: Let’s turn to Rust’s enum feature to add another tool to your toolbox.