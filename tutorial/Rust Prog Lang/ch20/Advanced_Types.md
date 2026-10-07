# Advanced Types

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66" valign="middle">
      <img
        src="../../../images/ed-images/ed_hack_1.png"
        width="40"
        height="40"
        alt="Author Git Logo"
        align="center"
      >
    </td>
    <td>
   <strong>
    Note: this chapter is hard to condense you might want to just read it.
    </strong> 
    </td>
  </tr>
</table>

[Advanced Types](https://doc.rust-lang.org/book/ch20-03-advanced-types.html)

## Super Summary Code

```rust

//-------------- Implementing External Traits with the Newtype Pattern-------------- (section copied from advanced traits)
// The orphan rule states we’re only allowed to implement a trait on a type if either the trait or the type, or both, are local to our crate. 
// It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct which will have one field and be a thin wrapper around the type for which we want to implement a trait.
// There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.
// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding.

use std::fmt;

struct Wrapper(Vec<String>); // newType

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

// Another example of newtype to make arbitrary types enforceable:
use std::ops::Add;

struct Millimeters(u32); // newtype 
struct Meters(u32); // newtype

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

//-------------- Type Synonyms and Type Aliases --------------
// unlike the Millimeters and Meters types we created in Listing 20-16, Kilometers is not a separate, new type.
type Kilometers = i32; // Now the alias Kilometers is a synonym for i32

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y); //alias can be used with the same type
// However, we don’t get the type-checking benefits that we get from the newtype pattern discussed earlier. 

// The main use case for type synonyms is to reduce repetition
// Note how tedious it is: Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error-prone.
Box<dyn Fn() + Send + 'static>

fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
    }
}
// vs this
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }


// Type aliases are also commonly used with the `Result<T, E>` type for reducing repetition. 
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
//vs
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}


//-------------- The Never Type That Never Returns --------------
// Rust has a special type named ! that’s known in type theory lingo as the `empty` type because it has no values. 
// - The formal way of describing this behavior is that expressions of type ! can be coerced into any other type. 
fn bar() -> ! {
    // --snip--
}
// essentially this type is used in keywords and other places where you want rust compiler to omit the type on the return of conditionals
// - continue uses it under the hood (so that we dont get error from Err not returning an u32)
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

// It's used in the panic! macro as the value of the expression.
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
} // it's coerced into T type so the overall match equates to T
// It's used in the loop keyword as the value of the expression
loop {
    print!("and ever ");
} // Here, the loop never ends, so ! is the value of the expression.


//-------------- Dynamically Sized Types and the Sized Trait --------------
// - Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.
// - Example of a dynamically sized type called str, not &str, but str on its own, is a DST. 
// When storing text entered by a user, we can’t know how long the string is until runtime which means we can’t create a variable of type str, nor can we take an argument of type str. 
let s1: str = "Hello there!"; // 12 bytes of storage 
let s2: str = "How's it going?"; // needs 15

// We make the type of s1 and s2 string slice (&str) rather than str.
//  So, although &T is a single value that stores the memory address of where the T is located, a string slice is two values: the address of the str and its length. As such, we can know the size of a string slice value at compile time: It’s twice the length of a usize.
// In general, this is the way in which dynamically sized types are used in Rust
// ** The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
// We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>
// Every trait is a dynamically sized type we can refer to by using the name of the trait. 
// we mentioned that to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).

// To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:
fn generic<T>(t: T) {
    // --snip--
}
// is actually treated as though we had written this:
fn generic<T: Sized>(t: T) {
    // --snip--
}
// By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
fn generic<T: ?Sized>(t: &T) { //?Sized means “T may or may not be Sized
    // --snip--
}
// and this notation overrides the default that generic types must have a known size at compile time. The ?Trait syntax with this meaning is only available for Sized, not any other traits.
// Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.
```

## Super Summary Info - For sections with no code examples
The newtype pattern is also useful for:
- statically enforcing that values are never confused and indicating the units of a value (Millimeters and Meters structs wrapped u32 values in a newtype (last chapter) - enforces abstract named types) 
- abstract away some implementation details of a type: The new type can expose a public API that is different from the API of the private inner type.
- hide internal implementation - like a People type to wrap a HashMap<i32, String> that stores a person’s ID associated with their name with only public method to add a name string to the collection - but has private implementation internally.



## Advanced Types

- The Rust type system has some features that we’ve so far mentioned but haven’t yet discussed.
- We’ll start by discussing newtypes in general as we examine why they are useful as types.
- Then, we’ll move on to type aliases, a feature similar to newtypes but with slightly different semantics.
- We’ll also discuss the ! type and dynamically sized types.

### Type Safety and Abstraction with the Newtype Pattern

- This section assumes you’ve read the earlier section “Implementing External Traits with the Newtype Pattern”.
- The newtype pattern is also useful for tasks beyond those we’ve discussed so far, including statically enforcing that values are never confused and indicating the units of a value.
- You saw an example of using newtypes to indicate units. Recall that the Millimeters and Meters structs wrapped u32 values in a newtype.
- If we wrote a function with a parameter of type Millimeters, we wouldn’t be able to compile a program that accidentally tried to call that function with a value of type Meters or a plain u32.

- We can also use the newtype pattern to abstract away some implementation details of a type: The new type can expose a public API that is different from the API of the private inner type.

- Newtypes can also hide internal implementation.
- For example, we could provide a People type to wrap a `HashMap<i32, String>` that stores a person’s ID associated with their name.
- Code using People would only interact with the public API we provide, such as a method to add a name string to the People collection;
  - that code wouldn’t need to know that we assign an i32 ID to names internally.
- The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details, which we discussed in the “Encapsulation that Hides Implementation Details” section in Chapter 18.

### Type Synonyms and Type Aliases

Rust provides the ability to declare a type alias to give an existing type another name. For this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:

```rust
    type Kilometers = i32;
```

- Now the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters types we created in Listing 20-16, Kilometers is not a separate, new type. 
- Values that have the type Kilometers will be treated the same as values of type i32:

```rust
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
```

- Because Kilometers and i32 are the same type, we can add values of both types and can pass Kilometers values to functions that take i32 parameters. 
- However, using this method, we don’t get the type-checking benefits that we get from the newtype pattern discussed earlier. 
- In other words, if we mix up Kilometers and i32 values somewhere, the compiler will not give us an error.

- The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:

```rust
Box<dyn Fn() + Send + 'static>
```
- Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error-prone. 
- Imagine having a project full of code like that in Listing 20-25.

```rust
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
    }
```

- This code is much easier to read and write! 
- Choosing a meaningful name for a type alias can help communicate your intent as well (thunk is a word for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored).

- Type aliases are also commonly used with the `Result<T, E>` type for reducing repetition. 
- Consider the std::io module in the standard library. 
- I/O operations often return a `Result<T, E>` to handle situations when operations fail to work. 
- This library has a std::io::Error struct that represents all possible I/O errors. 
- Many of the functions in std::io will be returning `Result<T, E>` where the E is std::io::Error, such as these functions in the Write trait:

```rust
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```
The Result<..., Error> is repeated a lot. As such, std::io has this type alias declaration:

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

- Because this declaration is in the std::io module, we can use the fully qualified alias std::io::Result<T>; that is, a Result<T, E> with the E filled in as std::io::Error. 
- The Write trait function signatures end up looking like this:

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```
- The type alias helps in two ways: It makes code easier to write and it gives us a consistent interface across all of std::io. 
- Because it’s an alias, it’s just another Result<T, E>, which means we can use any methods that work on Result<T, E> with it, as well as special syntax like the ? operator.

### The Never Type That Never Returns

Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values. We prefer to call it the never type because it stands in the place of the return type when a function will never return. Here is an example:

```rust
fn bar() -> ! {
    // --snip--
}
```

- This code is read as “the function bar returns never.” 
- Functions that return never are called diverging functions. 
- We can’t create values of the type !, so bar can never possibly return.

- But what use is a type you can never create values for? 
- Recall the code from Listing 2-5, part of the number-guessing game; we’ve reproduced a bit of it here in Listing 20-27.

```rust
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

- At the time, we skipped over some details in this code. 
- In “The match Control Flow Construct” section in Chapter 6, we discussed that match arms must all return the same type. 
- So, for example, the following code doesn’t work:

```rust
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };
```

- The type of guess in this code would have to be an integer and a string, and Rust requires that guess have only one type. 
- So, what does continue return? How were we allowed to return a u32 from one arm and have another arm that ends with continue in Listing 20-27?

- As you might have guessed, continue has a ! value. 
- That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value. 
- Because ! can never have a value, Rust decides that the type of guess is u32.

- The formal way of describing this behavior is that expressions of type ! can be coerced into any other type. 
- We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.

- The never type is useful with the panic! macro as well. 
- Recall the unwrap function that we call on Option<T> values to produce a value or panic with this definition:

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

- In this code, the same thing happens as in the match in Listing 20-27: 
    - Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T. 
- This code works because panic! doesn’t produce a value; 
    - it ends the program. In the None case, we won’t be returning a value from unwrap, so this code is valid.

- One final expression that has the type ! is a loop:

```rust
    print!("forever ");

    loop {
        print!("and ever ");
    }
```
- Here, the loop never ends, so ! is the value of the expression. 
- However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.

### Dynamically Sized Types and the Sized Trait

- Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type. 
- This leaves one corner of its type system a little confusing at first: the concept of dynamically sized types. 
- Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.

- Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book. 
- That’s right, not &str, but str on its own, is a DST. 
- In many cases, such as when storing text entered by a user, we can’t know how long the string is until runtime. 
- That means we can’t create a variable of type str, nor can we take an argument of type str. 
- Consider the following code, which does not work:

```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

- Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory. 
- If Rust allowed us to write this code, these two str values would need to take up the same amount of space. 
- But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15. 
- This is why it’s not possible to create a variable holding a dynamically sized type.

- So, what do we do? In this case, you already know the answer: 
- We make the type of s1 and s2 string slice (&str) rather than str. 
- Recall from the “String Slices” section in Chapter 4 that the slice data structure only stores the starting position and the length of the slice. 
- So, although &T is a single value that stores the memory address of where the T is located, a string slice is two values: 
- the address of the str and its length. 
- As such, we can know the size of a string slice value at compile time: 
- It’s twice the length of a usize. 
- That is, we always know the size of a string slice, no matter how long the string it refers to is. 
- In general, this is the way in which dynamically sized types are used in Rust: 
- They have an extra bit of metadata that stores the size of the dynamic information. 
- The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

- We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>. 
- In fact, you’ve seen this before but with a different dynamically sized type: traits. 
- Every trait is a dynamically sized type we can refer to by using the name of the trait. 
- In the “Using Trait Objects to Abstract over Shared Behavior” section in Chapter 18, we mentioned that to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).

- To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. 
- This trait is automatically implemented for everything whose size is known at compile time. 
- In addition, Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:

```rust
fn generic<T>(t: T) {
    // --snip--
}
```
is actually treated as though we had written this:
```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```
By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

- A trait bound on ?Sized means “T may or may not be Sized,” and this notation overrides the default that generic types must have a known size at compile time. 
- The ?Trait syntax with this meaning is only available for Sized, not any other traits.

- Also note that we switched the type of the t parameter from T to &T. 
- Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.