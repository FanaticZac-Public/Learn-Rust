# Concise Control Flow with if let and let...else

## Super Summary
```rust
//-------------- Matches Are Exhaustive --------------
// The 'if let' syntax is a less verbose way to handle values that match one pattern while ignoring the rest. 
// Regular way
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
// 'if let' way - note no match keyword required - just condition
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}

// We can also in include else:
// normal way
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
// 'if let' way 
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}


//-------------- “Happy Path” with let...else --------------
// The common pattern is to perform some computation when a value is present and return a default value otherwise. 
// Normal way
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}
// 'if let' way 
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
// - this could get complex fast (the above) with all those nested conditions...


// But here is a better way!
//  We could also take advantage of the fact that expressions produce a value either to produce the state from the if let or to return early (You could do something similar with a match, too.)
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
// - frankly for this - original match was the better way...


// But wait - there is a way out of the muck!
// The "let...else" syntax (long way to go)
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // state being the inferred variable for anything else 
    if state.existed_in(1900) { 
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
// Notice that it stays on the “happy path” in the main body of the function this way, without having significantly different control flow for two branches the way the if let did.
```

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

