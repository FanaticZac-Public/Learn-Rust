# Enums (enumerations) and Pattern Matching

Topics: 
- Define a Enum type by enumerating its possible variants. 
- How an enum can encode meaning along with data. 
- Explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing. 
- Show how pattern matching in the match expression makes it easy to run different code for different values of an enum. 
- How the if let construct is another convenient and concise idiom available to handle enums in your code.

This chapter contains: 
- [Defining an Enum](./Enum_Definition.md)
- [The match Control Flow Construct](Match_Construct.md)
- [Concise Control Flow with if let and let...else](./If-Let_&_Let_Else.md)

## Summary

Covers how to use enums to create custom types that can be one of a set of enumerated values, how the standard library’s Option<T> type helps you use the type system to prevent errors. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety: The compiler will make certain your functions only get values of the type each function expects.
