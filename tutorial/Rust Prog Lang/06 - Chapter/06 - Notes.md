# Chapter 06 - Enums and Pattern Matching

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

## The match Control Flow with if let and let...else

'match' that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

- **Rust** The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

We can write a function that takes an unknown US coin and, in a similar way as the counting machine, determines which coin it is and returns its value in cents

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }

    // - can have multiline with curly boys (aka rust says 'arms')
    // Coin::Penny => {
        //     println!("Lucky penny!");
        //     1
        // }
}
```

- '=>' just separates the pattern and code to run
- (just a switch..)
-

### Patterns That Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

- Example shows collector coin values that vary

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state. Then, we can use state in the code for that arm, like so:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska).
-When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state). At that point, the binding for state will be the value UsState::Alaska. We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

### The Option<T> match Pattern

In the previous section, we wanted to get the inner T value out of the Some case when using Option<T>

- Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

- This sets variables correctly using the match conditions - and by knowing there's a null value - compiler can check for some and only execute expression if so - avoiding the primary errors to do with code execution on null values in other languages.

### Matches Are Exhaustive

There’s one other aspect of match we need to discuss: The arms’ patterns must cover all possibilities. - so no dead ends!

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }
```

- error[E0004]: non-exhaustive patterns: `None` not covered
- Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.

### Catch-All Patterns and the \_ Placeholder

- we can also take special actions for a few particular values, but for all other values take one default action
-

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => move_player(),
        // _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

```

- for the last variable, the variable name can be anything so long as it's last it will act as the catch all.
- if we use '\_' however, it will catch all so long as we don't need to pass that value to function.
- Reminder - Rust makes it so all variables are accounted for
- "\_ => ()," This pattern in example would mean nothing happens (empty tuple technically - but it's accounted for 'the arm')

## Concise Control Flow with if let and let...else

The 'if let' syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // prints out The maximum is configured to be 3

    // but here is 'if let'
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```

- Using if let means less typing, less indentation, and less boilerplate code.
- you lose the exhaustive checking match enforces that ensures that you aren’t forgetting to handle any cases.
- In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

  We can include an else with an if let.

```rust
   let mut count = 0;
   match coin {
       Coin::Quarter(state) => println!("State quarter from {state:?}!"),
       _ => count += 1,
   }

   // or with else

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
```
- The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression
- I don't see any real advantage but sure.

### Staying on the “Happy Path” with let...else

The common pattern is to perform some computation when a value is present and return a default value otherwise.
- if we wanted to say something funny depending on how old the state on the quarter was, we might introduce a method on UsState to check the age of a state, like so:

```rust 
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
```

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
```
- That gets the job done, but it has pushed the work into the body of the if let statement, and if the work to be done is more complicated, it might be hard to follow exactly how the top-level branches relate.

We could also take advantage of the fact that expressions produce a value either to produce the state from the if let or to return early

```rust
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```
- This is a bit annoying to follow in its own way, though! One branch of the if let produces a value, and the other one returns from the function entirely.
- *(I hate this pattern - not going to lie)*

Now let's solve this contrived problem to absurd switch syntax:

```rust

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```
- Notice that it stays on the “happy path” in the main body of the function this way, without having significantly different control flow for two branches the way the if let did.
- If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let and let...else are in your Rust toolbox as well.
- *yuck*

## Summary
We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.
