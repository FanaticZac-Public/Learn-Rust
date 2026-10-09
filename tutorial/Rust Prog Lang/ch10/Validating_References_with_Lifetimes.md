# Validating References with Lifetimes

## Super Summary

```rust
//-------------- Dangling References --------------
// Example
fn main() {
    // outer scope
    let r;

    { // inner scope
        let x = 5;
        r = &x; //reference given to outer scope variable
    } // scope ends

    // Dangled!!!! noOOOo.

    println!("r: {r}"); 
    // error[E0597]: `x` does not live long enough
}

//-------------- The Borrow Checker --------------
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid. 
// This shows annotations showing the lifetimes of the variables.
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}   // `b (the second variable) is shorter than `a - dangled

// fixes the code so that it doesn’t have a dangling reference and it compiles without any errors.
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
// `a is shorter than `b - so lifetime is reasonable


//-------------- Generic Lifetimes in Functions --------------
// Example 2 
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
    // The longest string is abcd
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
} 
// error[E0106]: missing lifetime specifier
// - The compiler (or us) don’t know if the 'if block' in the body of this function returns a reference to x or a reference to y.
// - therefore it can't determine if result will last longer than the reference at compile time.


//-------------- Lifetime Annotation Syntax --------------
// Lifetime annotations describe the relationships of the lifetimes of multiple references to each other
// Functions can accept references with any lifetime by specifying a generic lifetime parameter.

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

// One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.

//-------------- In Function Signatures --------------
// Note the generic spot as well. 
// Note signature only (called lifetime contract)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// This tells the compiler to treat both variables as though they have the same lifetime (using whichever is shortest aka (first example))

// Example:
fn main() {
    let string1 = String::from("long string is long");

    { // inner scope
        let string2 = String::from("xyz"); // shorter reference
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    } // inner scope end - shorter reference lifetime end

    // Compiler knows that last usage of result variable was within the shortest lifetime scope (printing of result)
    // if print was here instead:
    // println!("The longest string is {result}");
    // error[E0597]: `string2` does not live long enough
} 

//-------------- Relationships --------------
// The way in which you need to specify lifetime parameters depends on what your function is doing. 
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
} // no lifetime on the y because we don't return it

// no lifetime on params - but is on return type
// because we're creating a variable and returning it 
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
// However we still get error because result goes out of scope then
// error[E0515]: cannot return value referencing local variable `result`

//-------------- In Struct Definitions--------------
// We can define structs to hold references, but we would need to add a lifetime annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> { // Note - same a generic placement
    part: &'a str,
    // example would be better with multiple fields because:
    // we're saying they must all be valid for shortest scope similarly as previous example for the struct to be safe 
    // part2: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap(); //ref
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

//-------------- Lifetime Elision --------------
// This works without explicit lifelines - why?
// Because in early Rust it because repetitive and pedantic
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// Rust adds it at compile time for some functions: 
// fn first_word<'a>(s: &'a str) -> &'a str {

// It follows these 3 rules:
// 1. compiler assigns a lifetime parameter to each parameter that’s a reference.
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
// - This third rule makes methods much nicer to read and write because fewer symbols are necessary.

//-------------- In Method Definitions --------------
// For methods - you also have to place on the impl (like generics)
impl<'a> ImportantExcerpt<'a> {
    // the first and second rule also applies here so there's no lifelines on method signatures - just the outer impl line
    fn level(&self) -> i32 {
        3
    }
}

// third lifetime elision rule applies and self used and returned
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
// in short no lifelines required on methods other than outer impl signature


//-------------- The Static Lifetime --------------
// All string literals have the 'static lifetime because they are stored in the program's binary:
let s: &'static str = "I have a static lifetime.";
let s = "I have a static lifetime. "; // same

// anyway a function might ask for this: 
fn keep_for_program(s: &'static str) {} // only declared string literals

// Not allowed example
let owned = String::from("hello");
let borrowed: &str = &owned; // valid only while `owned` is alive
keep_for_program(borrowed); 
// error[E0597]: `i` does not live long enough ... "argument requires that `i` is borrowed for `'static`"
// Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is to fix those problems, not to specify the 'static lifetime.
```

## Validating References with Lifetimes



Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every reference in Rust has a lifetime, which is the scope for which that reference is valid.

- Usually inferred like how types are

In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure that the actual references used at runtime will definitely be valid.

Annotating lifetimes is not even a concept most other programming languages have, so this is going to feel unfamiliar. Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so that you can get comfortable with the concept.

### Dangling References

The main aim of lifetimes is to prevent dangling references, which, if they were allowed to exist, would cause a program to reference data other than the data it’s intended to reference.

```rust
fn main() {
    // outer scope
    let r;

    { // inner scope
        let x = 5;
        r = &x;
    }

    // Dangled!!!! noOOOo.

    println!("r: {r}");
}

// would not be a dangle in Rust because compiler would step in.
// error[E0597]: `x` does not live long enough
// However we still have to manage this...
```

So, how does Rust determine that this code is invalid? It uses a borrow checker.

### The Borrow Checker

The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
```

- r has lifetine `a that exits to end of function
- x has lifetime `b to end of it's scope
- Rust sees that r being give reference to value with lifetime smaller \`b is shorter than \`a

Fixed:

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```

-- \`b is longer than \`a

Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure that references will always be valid, let’s explore generic lifetimes in function parameters and return values.

### Generic Lifetimes in Functions

We’ll write a function that returns the longer of two string slices. This function will take two string slices and return a single string slice.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
    // The longest string is abcd
}
```

- Note that we want the function to take string slices, which are references, rather than strings, because we don’t want the longest function to take ownership of its parameters.

If we try to implement the longest function as shown in Listing 10-20, it won’t compile.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// error[E0106]: missing lifetime specifier
```

- The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y.
- To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so that the borrow checker can perform its analysis.

### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references live.

Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

Syntax

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

//  Most people use the name 'a for the first lifetime annotation.
```

We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.

### In Function Signatures

To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.

We want the signature to express the following constraint: The returned reference will be valid as long as both of the parameters are valid. This is the relationship between lifetimes of the parameters and the return value.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a

The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.

In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.

These relationships are what we want Rust to use when analyzing this code.

when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.

The lifetime annotations become part of the contract of the function, much like the types in the signature.

Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler.

If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the constraints more precisely.

If, instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to a use of our code many steps away from the cause of the problem.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes.

```rust
fn main() {
   let string1 = String::from("long string is long");

   {
       let string2 = String::from("xyz");
       let result = longest(string1.as_str(), string2.as_str());
       println!("The longest string is {result}");
   }

   // So this is what is mean by shortest returned
   // if the shortest lifeline return string2 is equal to the
   // scope of the usage (of result) - it's ok.
}
```

In this example, string1 is valid until the end of the outer scope

string2 is valid until the end of the inner scope

and result references something that is valid until the end of the inner scope

un this code and you’ll see that the borrow checker approves; it will compile and print The longest string is long string is long.

Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments.

We’ll move the declaration of the result variable outside the inner scope but leave the assignment of the value to the result variable inside the scope with string2. Then, we’ll move the println! that uses result to outside the inner scope, after the inner scope has ended. The code in Listing 10-23 will not compile.

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {result}");

    // error[E0597]: `string2` does not live long enough

    // So this the usage of result is longer than the shorter
    // lifeline of string2 - therefore the checker will error.
}
```

The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a.

As humans, we can look at this code and see that string1 is longer than string2, and therefore, result will contain a reference to string1. Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the println! statement. However, the compiler can’t see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. Therefore, the borrow checker disallows the code in Listing 10-23 as possibly having an invalid reference.

Experiment with this.

### Relationships

The way in which you need to specify lifetime parameters depends on what your function is doing.

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. The following code will compile:

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
x
}

We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()

    // error[E0515]: cannot return value referencing local variable `result`
}
```

- This returns a dangling reference to result and there's no way here to apply a lifetime parameter such that variable is withing function. It doesn't matter if we have that in the return type
- We could return an owned variable, and should.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

### In Struct Definitions

So far, the structs we’ve defined all hold owned types. We can define structs to hold references, but in that case, we would need to add a lifetime annotation on every reference in the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

### Lifetime Elision

You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references. However, we had a function in Listing 4-9, shown again in Listing 10-25, that compiled without lifetime annotations.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Rust will automatically add lifetime parameters for the following, and if it still can't determine safety it will error: 

1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
    - This third rule makes methods much nicer to read and write because fewer symbols are necessary.

### In Method Definitions

When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters

Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.

Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name because those lifetimes are part of the struct’s type.

In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures. Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-24.

```rust 
// struct - i added this for compare
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// book example - lifetimes no necessary inside due to elision rules
impl<'a> ImportantExcerpt<'a> { // but is required here.
    fn level(&self) -> i32 { // not required for self rule 3.
        3
    }
}

// 
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.

Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.

### The Static Lifetime

One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.

All string literals have the 'static lifetime, which we can annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```
The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.

CAUTION: You might see suggestions in error messages to use the 'static lifetime. But before specifying 'static as the lifetime for a reference, think about whether or not the reference you have actually lives the entire lifetime of your program, and whether you want it to. Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is to fix those problems, not to specify the 'static lifetime.
### Generic Type Parameters, Trait Bounds, and Lifetimes

Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
```
- Ok, so it looks like combined im doing my lifetime syntax and generic syntax together. 
- lifetimes for the double inputs, and T for the 3rd, 
- Returning lifeline string slice.
- where it has the trait display
- printing ann
- returning x or y like the previous example.

Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.