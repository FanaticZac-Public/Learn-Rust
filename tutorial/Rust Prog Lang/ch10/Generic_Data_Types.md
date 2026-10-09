# Generic Data Types

## Super Summary

```rust
//-------------- In Function Definitions --------------
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

// However this code will get an error because of the < comparison:

// - error[E0369]: binary operation `>` cannot be applied to type `&T`
// - To correct - we need to use the trait PartialOrd, because our comparison won't work on all potential types that could be used (specifically the char type that has numeric values underneath)

use std::cmp::PartialOrd;

fn largest<T>(list: &[T]) -> &T {...}


//-------------- In Struct Definitions --------------
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

// setting struct to 2 types with 2 different parameters in <>
struct Point<T, U> {
    x: T,
    y: U,
}

// This will work because types are defined
let both_integer = Point { x: 5, y: 10 };
let both_float = Point { x: 1.0, y: 4.0 };
let integer_and_float = Point { x: 5, y: 4.0 };

//-------------- In Enum Definitions --------------
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

//-------------- In Method Definitions --------------

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

//-------------- Performance of Code Using Generics --------------
// The good news is that using generic types won’t make your program run any slower than it would with concrete types.
// Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
// In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

// This:
let integer = Some(5);
let float = Some(5.0);

// becomes this - during compile time:
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

## Generic Data Types
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
    let both_float
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
