# Defining an Enum

## Super Summary

```rust
//-------------- Enum(eration) Definition --------------
// Defining
enum IpAddrKind {
    V4,
    V6,
}

// Getting Values using double colon '::'
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// Can then define function signature to except a Type
fn route(ip_kind: IpAddrKind) {}

// Calling function
route(IpAddrKind::V4);
route(IpAddrKind::V6);

// Setting unique variables is concise then (more easy then struct in this case)
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

// Another advantage is each variant can have different types and amounts of data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// Can use it for these advantages but then have structs (or anything else) as the data structure underneath.
// - this is the actual API for IpAddr in Rust

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// See various types in one - Good for defining options of choice
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// structs would be less concise and uncoupled - therefore you would have to define function to handle all separately
struct QuitMessage; // unit struct
struct MoveMessage { x: i32, y: i32 }
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// We’re able to define methods on enums (like on structs)
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

//-------------- The Option Enum --------------
// The Option Enum is defined by the standard library to work in place of a null function in a typical language
// - Null value implementations in other languages cause bugs 
// In Rust - we use the Option value because it is an enum that allows various types (generics) to return None, or Some value.
enum Option<T> {
    None,
    Some(T),
}

// Rust can infer these types because we’ve specified a value inside the Some variant. 
// Value is stored inside Some()
let some_number = Some(5); // Option<i32>
let some_char = Some('e'); // Option<char>

// Rust requires us to annotate the overall Option type because it can't infer from None. 
let absent_number: Option<i32> = None;

// You can't use the variable with Some directly:
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
// error[E0277]: cannot add `Option<i8>` to `i8`
// - You can get value out when you need
// e.g. 
let sum = x + y.unwrap(); // there are many methods in Option on API, go check.

// In Rust: Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null. 
// - When you know that you will have an option null potential - you need to use enum Option<>
// In Rust - when using match to check for this enum value - you will be forced by the compiler to handle every case offered by an enum - thereby improving safety.
```

## Defining an Enum

- enums give you a way of saying a value is one of a possible set of values.
- we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle
- Currently, two major standards are used for IP addresses: version four and version six.
- we can enumerate all possible variants, which is where enumeration gets its name.
- Any IP address can be either a version four or a version six address, but not both at the same time.
  - That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants.

### Enum Values

```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // with structs
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

- this could be done with a struct however enum is more concise in usage (below):

```rust
    // with just enum
    enum IpAddr {
            V4(String),
            V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

- We attach data to each variant of the enum directly, so there is no need for an extra struct.
- The name of each enum variant that we define also becomes a function that constructs an instance of the enum.
- There’s another advantage to using an enum rather than a struct: Each variant can have different types and amounts of associated data.
  - Version four IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct.

```rust
   enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```

- The actual standard library definition is just this:
  - [IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html)

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

- you can put any kind of data inside an enum variant: strings, numeric types, or structs, other enums, etc.

Another example

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// could be done with structs
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

- Could be done with structs but each has it's own type - and it would be harder to manage with another function using input of several different types rather than 1 enum
  - function can just take message enum - and use as required

```rust
   impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

- Much easier to manage message functions using enum here.

### The Option Enum

This section explores a case study of Option, which is another enum defined by the standard library. The Option type encodes the very common scenario in which a value could be something, or it could be nothing.
**Rust special below**

- if you request the first item in a non-empty list, you would get a value. If you request the first item in an empty list, you would get nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.
  - Programming language design is often thought of in terms of which features you include, but the features you exclude are important too. Rust doesn’t have the null feature that many other languages have.
  - Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.
- In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, had this to say:
  - I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
- The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.
- However, the concept that null is trying to express is still a useful one: A null is a value that is currently invalid or absent for some reason.
  The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>, and it is defined by the standard library as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

- The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: You can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
  Here are some examples of using Option values to hold number types and char types:

```rust
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
```

- the other 2 can be inferred by value
- For absent_number, Rust requires us to annotate the overall Option type
  - The compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
- So, why is having Option<T> any better than having null?
  - Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value.
- For example, this code won’t compile, because it’s trying to add an i8 to an Option<i8>:

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    // error[E0277]: cannot add `Option<i8>` to `i8`
```

- \*\*The value here is that we've not pushed a null value to the compiler to prevent a bad value like if it were in 'null' in other languages.
- you have to convert an Option<T> to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
- In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.
  - Eliminating the risk of incorrectly assuming a not-null value helps you be more confident in your code.
  - Then, when you use that value, you are required to explicitly handle the case when the value is null.
  - Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
  - This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

- In general, in order to use an Option<T> value, you want to have code that will handle each variant. some value and none value. Match will be used for this
