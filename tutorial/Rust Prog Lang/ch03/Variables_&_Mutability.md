# Variables (Mutability), Constants, Shadowing

## Super Summary

```rust
fn main() {
    //-------------- Declaring immutable variable --------------
    let x = 5;
    println!("The value of x is: {x}");
    // error causing
    x = 6;
    println!("The value of x is: {x}");
    // error[E0384]: cannot assign twice to immutable variable `x`


    //-------------- Declaring Constants --------------
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    //-------------- Shadowing Example --------------
    let x = 5;

    let x = x + 1; // shadowing repeats initialization syntax
    // NEW variable created (same name) until loses scopes

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // The value of x in the inner scope is: 12
    } // loses scope

    println!("The value of x is: {x}");
    // The value of x is: 6


}
```

## Variables and Mutability

In Rust: 
- Variables are immutable by default (For safety and concurrency)
    - Can still choose mutability manually with `mut`

Advantages: 
- compiler knows what's up
- code is easier to reason through (informative)
    - bugs are easier to find

## Constants
- Constants are immutable too, obviously
    - you cannot use `mut`
    - you use `const` instead of `let`
    - the type must be annotated
    - const can be declared in any scope, including global
    - must be set to a value and the result of a function
        - not the result of an expression that could only be computed at runtime (can be expression of other constants)
    - naming convention is all caps with underscores for spaces
    - are valid for full entire program execution time, within the same scope as declared
        - makes useful for globally relevant data 

## Shadowing
- Shadowing occurs where the same immutable variable name is re-defined after the first 'overshadowing' it.
    - The first is 'shadowed' by the second (terminology)
    - Note: that 'let' is used in both redefinition
    - We are not changing the value - we are creating a new one with the same name
    - By using let we can perform new transformation on a value but have the variable remain immutable after
    - When the inner scope ends the shadowed value returns
    - We can also change the type of the value with shadowing
        - If we have a variable input as text - we could just shadow that count as an int without name changes like char_count, can just set new value with type - thus not requiring multiple variable names for each type
        - If we were to use to 'mut' for this we would get wrong type error