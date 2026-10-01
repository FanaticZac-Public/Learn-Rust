
# The match Control Flow with if let and let...else



## Super Summary
```rust
//-------------- match Control Flow Construct --------------
// Surprise - it's basically a switch
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Handles every potential possibility of coin and acts.
// => is what happens on match.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("Lucky quarter!");
            25
        } // you can bracket them and multi-line like this one.
    }
}


//-------------- Patterns That Bind to Values --------------
// match arms can bind to the parts of the values that match the pattern.
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --etc--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

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

// If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska)


//-------------- The Option<T> match Pattern --------------
// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value, otherwise return None.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5); // 6
let six = plus_one(five); // 7
let none = plus_one(None); // None


//-------------- Matches Are Exhaustive --------------
// The arms’ patterns must cover all possibilities.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
// error[E0004]: non-exhaustive patterns: `None` not covered
// Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Safety first - no loose arms.


//-------------- Catch-All Patterns and the _ Placeholder --------------
// Surprise! It's just a wild card. This catch-all pattern.
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other), // it infers the variable name here so long as there is a variable to catch all possible other cases
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
// This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed


// Underscore '_' is a special pattern that matches any value and does not bind to that value. (No variable required if no param needed)
let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}


// If you want nothing to happen:
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // Just use empty parenthesis 
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

```







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