<style>
@import url("../notes.css");
</style>

# Generic Types, Traits, and Lifetimes

Generics are abstract stand-ins for concrete types or other properties to reduce code duplication.

Functions can take parameters of some generic type, instead of a concrete type like i32 or String.

- Examples we've seen already are:
  - Option<T>
  - Vec<T>
  - HashMap<K, V>
  - Result<T, E>

Chapter discuss same techniques, making your own, how to use traits (to only accept certain behaviors), and lifelines feature.

lifetimes: a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure that references will be valid in more situations than it could without our help.

---

### In Function Definitions

```rust
// typical just for reference
fn largest_i32(list: &[i32]) -> &i32 {}

// generic function signature
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

- First <T> denotes a generic function
- T for type is common, but can be variable Type or whatever
- T is then used in normal place of type in signature and function

However this code will get an error because of the < comparison:

- error[E0369]: binary operation `>` cannot be applied to type `&T`
- To correct - we need to use the trait PartialOrd, because our comparison won't work on all potential types that could be used (specifically the char type that has numeric values underneath)
- [std::cmp::PartialOrd](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons)

```rust
use std::cmp::PartialOrd;

fn largest<T>(list: &[T]) -> &T {...}
```

### In Struct Definitions

```rust
// Setting struct to a type - means both in T are the same type
struct Point<T> {
    x: T,
    y: T,
}



fn main() {
    // now can hold int or float in struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // won't work because mismatched
    let wont_work = Point { x: 5, y: 4.0 };
    // error[E0308]: mismatched types

}
```

- <T, U> must be used if you want 2 types

```rust
// setting struct to 2 types with 2 different parameters in <>
    struct Point<T, U> {
        x: T,
        y: U,
    }

    // This will work because types are defined
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
```

- You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read.

### In Enum Definitions

```rust
// single generic type with familiar enum
enum Option<T> {
    Some(T),
    None,
}

// multiple generic type with familiar enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

### In Method Definitions

We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions too.

```rust
struct Point<T> {
    x: T,
    y: T,
}

//Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    // returns a reference to the data in the field x.
    println!("p.x = {}", p.x());
}
```

- Note that impl<T> and Point<T> (the impl would normally have a <T> for concrete, just as the Point wouldn't in it's struct)
- The Point<T> was already defined on the struct, but the impl is for it's own block to match, and not redundant

Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition.
impl<X1, Y1> Point<X1, Y1> {
    // The generic parameters X2 and Y2 are declared after fn mixup because they’re only relevant to the method.
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> { //different composition

    //  The method creates a new Point instance with the x value from the self Point (of type X1) and the y value from the passed-in Point (of type Y2).

        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    // takes x from p1 and 'c' from p2

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    // p3.x = 5, p3.y = c
}
```

The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition.

Keywords:
impl: Implement inherent or trait functionality.

### Performance of Code Using Generics

You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won’t make your program run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using generics at compile time.

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: One is i32 and the other is f64. As such, it expands the generic definition of Option<T> into two definitions specialized to i32 and f64, thereby replacing the generic definition with the specific ones.

The monomorphized version of the code looks similar to the following (the compiler uses different names than what we’re using here for illustration):

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

The generic Option<T> is replaced with the specific definitions created by the compiler. Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.

## 10.2 Defining Shared Behavior with Traits

A trait defines the functionality a particular type has and can share with other types.

We can use traits to define shared behavior in an abstract way.

We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

### Defining a Trait

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

For example, let’s say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds a news story filed in a particular location and a SocialPost that can have, at most, 280 characters along with metadata that indicates whether it was a new post, a repost, or a reply to another post.

We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or SocialPost instance. To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance. Listing 10-12 shows the definition of a public Summary trait that expresses this behavior.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- After the method signature, instead of providing an implementation within curly brackets, we use a semicolon

### Implementing a Trait on a Type

Now that we’ve defined the desired signatures of the Summary trait’s methods, we can implement it on the types in our media aggregator.

```rust
// Here is the trait that various related structures must implement independently - but for which have the available call.
pub trait Summary {
    fn summarize(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

// usage

```rust
use aggregator::{SocialPost, Summary};
// aggregator is just what we named the library crate

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
```

### Using Default Implementations

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // Default trait behavior
    }
}
```

To use a default implementation to summarize instances of NewsArticle, we specify an empty

impl block with impl Summary for NewsArticle {}.

Even though we’re no longer defining the summarize method on NewsArticle directly, we’ve provided a default implementation and specified that NewsArticle implements the Summary trait. As a result, we can still call the summarize method on an instance of NewsArticle, like this:

```rust
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // New article available! (Read more...)
```

Creating a default implementation doesn’t require us to change anything about the implementation of Summary on SocialPost in Listing 10-13. The reason is that the syntax for overriding a default implementation is the same as the syntax for implementing a trait method that doesn’t have a default implementation.

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. For example, we could define the Summary trait to have a summarize_author method whose implementation is required, and then define a summarize method that has a default implementation that calls the summarize_author method:

WE GET IT - ITS AN INTERFACE

### Traits - sort of started over but merged...

"Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior. In dynamically typed languages, we would get an error at runtime if we called a method on a type that didn’t define the method. But Rust moves these errors to compile time so that we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime, because we’ve already checked at compile time. Doing so improves performance without having to give up the flexibility of generics." - from final summary of trait section"

Show one code sample with with the varieties of implementation

Content

- Traits are basically interfaces (from other languages)
  - They can be set to enforce objects with the trait to create their own implementation
  - They can have default values
    - But can be overwritten. (overriding)
  - Traits are grouping of various function signatures (+/- implementations)
    - these function calls can also call each other
    - e.g. Trait Summary, fn summarize() can call sibling fn summarize_author() and summarize_content() as part of it's unified response.
    - however - it isn’t possible to call the default implementation from an overriding implementation of that same method.
  - Using Traits as Parameters
    - traits can be used to define functions that accept many different types.
    - It shows fn notify(item: &impl Summary){} as way to call print on item.summarize()
      - it specifies the impl keyword and the trait name
      - therefore it accepts any type implements the specific trait
        - to me this implies we can call function or even loop collections by the trait type rather than the actual item type
  - Trait Bound Syntax - The 'impl Trait' syntax
    - pub fn notify<T: Summary>(item: &T){}
      - Equivalent to 'Using Traits as Parameters' but with generic type parameter
    - The impl Trait syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound syntax can express more complexity in other cases.
    - pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    - -vs.
    - pub fn notify<T: Summary>(item1: &T, item2: &T) {}
  - Multiple Trait Bounds with the + Syntax
    - Say we wanted notify to use display formatting as well as summarize on item: We specify in the notify definition that item must implement both Display and Summary.
      - pub fn notify(item: &(impl Summary + Display)) {}
      - or
      - pub fn notify<T: Summary + Display>(item: &T) {}
    - With the two trait bounds specified, the body of notify can call summarize and use {} to format item
  - Clearer Trait Bounds with where Clauses
    - Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. (to make easier to read)
    - fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    - or with (where)
    - fn some_function<T, U>(t: &T, u: &U) -> i32
      where
      T: Display + Clone,
      U: Clone + Debug,
      {}

### Returning Types That Implement Traits

        - use the impl Trait syntax in the return position to return a value of some type that implements a trait
        - fn returns_summarizable() -> impl Summary {
            SocialPost { ... }
        }
        - Returns SocialPost - but function signature doesn't need to know that if we just want the trait method access on return
        - Returning by Trait is useful for closure and iterators because it reduces the amount of specificity and code to write them
            - However, you can only use impl Trait if you’re returning a single type.
            - Conditional returning NewsArticle or SocialPost in the body won't work
                - Due to how the compiler manages them (ch18 elaborates)
        - Using Trait Bounds to Conditionally Implement Methods
            - recall from the “Method Syntax” section of Chapter 5 that Self is a type alias for the type of the impl block

### Using Trait Bounds to Conditionally Implement Methods

- no way to shorten this one -

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Self is a type alias for the type of the impl block
impl<T> Pair<T> { // always impliments returns new Pair<T>
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

//  But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait.

Implementations of a trait on any type that satisfies the trait bounds are called _blanket implementations_ and are used extensively in the **Rust** standard library.

_blanket implementations_ - Implementations of a trait on any type that satisfies the trait bounds

For example, the standard library implements the ToString trait on any type that implements the Display trait. The impl block in the standard library looks similar to this code:

- e.g.

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.

```rust
let s = 3.to_string();
```

Blanket implementations appear in the documentation for the trait in the “Implementors” section.

New Keywords: - where: Denote clauses that constrain a type.

Links: -

## 10.3 Validating References with Lifetimes

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

    // Dangled!!!! noooooooooooOOoOOooooooooooOOOooo.

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
- Returning lifelined string slice.
- where it has the trait display
- printing ann
- returing x or y like the previous example.

Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

## Summary

We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations. Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

Believe it or not, there is much more to learn on the topics we discussed in this chapter: Chapter 18 discusses trait objects, which are another way to use traits. There are also more complex scenarios involving lifetime annotations that you will only need in very advanced scenarios; for those, you should read the Rust Reference. But next, you’ll learn how to write tests in Rust so that you can make sure your code is working the way it should.