<style>
@import url("../notes.css");
</style>

<div class="zac-note">
On this one - I made some good cheat sheet notes - compared to notes - but this is problematic. 
Going forward - I'm just going to do this:
- First pass write/copy notes into notes.md
- once per day - go back to the first incomplete cheat sheet and do that section
    - For cheat sheet, for each chapter do one section of cheat sheet as it's own thing - but unify the code and explanation
    - e.g. for this section, generics, Traits, and lifelines would each be their won.
- So consider this one needing a redo - but some of the effort here can be re-used (this was first attempt at doing this in one pass - but it's hard to learn and organize at the same time.)
</div>

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

### In Method Definitions

### Performance of Code Using Generics

Because Rust performs monomorphization of all generic types, it won’t make your program run any slower than it would with concrete types.

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. (Basically it will look at your code and create binary copies of each function, struct, enum, etc, for only the typed function that are actually being used with your generics functions in your code - at compile time). 

For more detail: [Performance of Code Using Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html?search=#performance-of-code-using-generics) 

## 10.2 Defining Shared Behavior with Traits
A trait is similar to an interface in other languages. 

Each type implementing this trait must provide its own custom behavior for the body of the method.

The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
```rust
pub trait Summary {
    // this is src/lib.rs 
    // semi-colon ending - objects with trait implement their own logic.
    fn summarize(&self) -> String;
}
```
- After the method signature, instead of providing an implementation within curly brackets, we use a semicolon
### Defining a Trait

```rust
    // this is src/lib.rs 
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

// NewsArticle implements Summary trait to match it's unique structure
// Note syntax here for trait:
// -  impl Summary for BLANK
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

// SocialPost implements Summary trait to match it's unique structure
// Note syntax here for trait:
// -  impl - Summary for - BLANK
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
Example Usage:
```rust
//  this is main.rs (binary crate) importing 
//  - aggregator crate from (library crate) src/lib.rs 
use aggregator::{SocialPost, Summary};

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

### Using Traits as Parameters

### Returning Types That Implement Traits

### Using Trait Bounds to Conditionally Implement Methods

## 10.3 Validating References with Lifetimes

### Dangling References

### The Borrow Checker

### Generic Lifetimes in Functions

### Lifetime Annotation Syntax

### In Function Signatures

### Relationships

### In Struct Definitions

### Lifetime Elision

### In Method Definitions

### The Static Lifetime

### Generic Type Parameters, Trait Bounds, and Lifetimes

## Summary
