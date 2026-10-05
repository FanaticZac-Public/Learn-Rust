# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple. Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow. A pattern consists of some combination of the following:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

- Some example patterns include x, (a, 3), and Some(Color::Red). 
- In the contexts in which patterns are valid, these components describe the shape of data. 
- Our program then matches values against the patterns to determine whether it has the correct shape of data to continue running a particular piece of code.

- To use a pattern, we compare it to some value. 
- If the pattern matches the value, we use the value parts in our code. 
- Recall the match expressions in Chapter 6 that used patterns, such as the coin-sorting machine example. 
- If the value fits the shape of the pattern, we can use the named pieces. 
- If it doesn’t, the code associated with the pattern won’t run.

- This chapter is a reference on all things related to patterns. - - We’ll cover the valid places to use patterns, the difference between refutable and irrefutable patterns, and the different kinds of pattern syntax that you might see. 
- By the end of the chapter, you’ll know how to use patterns to express many concepts in a clear way.

Chapter 19 contains:

- [All the Places Patterns Can Be Used](./Where_to_use_Patterns.md) - Done (super summary perfect)
- [Refutability: Whether a Pattern Might Fail to Match](./Refutability_with_Patterns.md) - Done (super summary perfect)
- [Pattern Syntax](./Pattern_Syntax.md) - Done (super summary perfect)

## Summary

Rust’s patterns are very useful in distinguishing between different kinds of data. When used in match expressions, Rust ensures that your patterns cover every possible value, or your program won’t compile. Patterns in let statements and function parameters make those constructs more useful, enabling the destructuring of values into smaller parts and assigning those parts to variables. We can create simple or complex patterns to suit our needs.

