# Defining Shared Behavior with Traits

## Super Summary

```rust
// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.


//-------------- Defining a Trait --------------
pub trait Summary {
    fn summarize(&self) -> String;
}


//-------------- Implementing a Trait on a Type --------------
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


//-------------- Using Default Implementations --------------
// you just fill in the function! 
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // Default trait behavior
    }
}

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


//-------------- Using Traits as Parameters --------------
// &impl Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ------ Trait Bound Syntax -------
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// you can see here that for many params it's much easier to type out
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
// vs (so long as they only need 1 type for both)
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// ------ Multiple Trait Bounds with the + Syntax -------
// the + sign means item must implement both Display and Summary
pub fn notify(item: &(impl Summary + Display)) {}
// or 
pub fn notify<T: Summary + Display>(item: &T) {}

// ------ Clearer Trait Bounds with where Clauses -------
// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So, instead of writing this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// we can use a where clause, like this:
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// ------ Returning Types That Implement Traits -------
// You can use trait bounds in a return type
// However, you can only use impl Trait if you’re returning a single type.
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
}

// ------ Using Trait Bounds to Conditionally Implement Methods -------
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. 
// called blanket implementations, they are used extensively in the Rust standard library.
impl<T: Display> ToString for T {
    // --snip--
}

// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
let s = 3.to_string();
```

## Defining Shared Behavior with Traits

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