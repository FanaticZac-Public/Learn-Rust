
# Closures

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.

Unlike functions, closures can capture values from the scope in which they’re defined. We’ll demonstrate how these closure features allow for code reuse and behavior customization.

## Capturing the Environment

We’ll first examine how we can use closures `||` to capture values from the environment they’re defined in for later use.

Here’s the scenario: Every so often, our T-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion.

People on the mailing list can optionally add their favorite color to their profile.

- If the person chosen for a free shirt has their favorite color set, they get that color shirt. If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) ->
    ShirtColor {
        // || is closure with no arguments (but could between pipes)
        // essentially a closer allows us to place a function call in the parameter of another function
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// OUTPUT:
// The user with preference Some(Red) gets Red
// The user with preference None gets Blue
```

One interesting aspect here is that we’ve passed a closure that calls self.most_stocked() on the current Inventory instance. The standard library didn’t need to know anything about the Inventory or ShirtColor types we defined, or the logic we want to use in this scenario.

The closure captures an immutable reference to the self Inventory instance and passes it with the code we specify to the unwrap_or_else method. Functions, on the other hand, are not able to capture their environment in this way.

## Inferring and Annotating Closure Types

Differences between closures and functions

Functions: - Type annotations are required on functions because the types are part of an explicit interface exposed to your users. - Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.

Closures:

- Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
- Closures aren’t used in an exposed interface like this: They’re stored in variables, and they’re used without naming them and exposing them to users of our library.
- Within these limited contexts, the compiler can infer the types of the parameters and the return type (like most variables)
  - there are rare cases where the compiler needs closure type annotations too.
  - we can add type annotations if we want to increase explicitness and clarity.
- Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.

```rust
// explicitly type annotated closure
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

// Showing how function and closure syntax are similar and different
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

// Similar to let v = Vec::new();
// - we need there to annotated types or variables provided at initialization to infer the type


// For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value.
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
let n = example_closure(5);
// error[E0308]: mismatched types
// - Because example_closure, while showing any type in it's definition, can only be used for 1 type later and cannot act as a wild card (just looks like that becuase it can infer based on first usage - but second usage and the crab will attack (error))
```

## Capturing References or Moving Ownership

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:

1. borrowing immutably,
2. borrowing mutably
3. taking ownership.

The closure will decide which of these to use based on what the body of the function does with the captured values.

```rust
fn main() {
    // Defining and calling a closure that captures an immutable reference

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Before defining closure: [1, 2, 3]

    let only_borrows = || println!("From closure: {list:?}");
    // Before calling closure: [1, 2, 3]

    println!("Before calling closure: {list:?}");
    // From closure: [1, 2, 3]

    only_borrows();
    println!("After calling closure: {list:?}");
    // After calling closure: [1, 2, 3]
}
// This example also illustrates that a variable can bind to a closure definition, and we can later call the closure by using the variable name and parentheses as if the variable name were a function name.

// Because we can have multiple immutable references to list at the same time, list is still accessible from the code before the closure definition, after the closure definition but before the closure is called, and after the closure is called. 
```
```rust
fn main() {
    // Defining and calling a closure that captures a mutable reference

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Before defining closure: [1, 2, 3]

    // Captures mutable ref and operates
    let mut borrows_mutably = || list.push(7);
    // borrow ends - imagine {scoped}

    // println!("print mutable borrow {list:?}");
    // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable


    borrows_mutably();
    println!("After calling closure: {list:?}");
    // After calling closure: [1, 2, 3, 7]
}

// Between the closure definition and the closure call, an immutable borrow to print isn’t allowed, because no other borrows are allowed when there’s a mutable borrow. 
```

If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the `move` keyword before the parameter list. (This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.)

```rust
// passing data to be owned by new thread using move and closures
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // this will move list ownership to new thread with closure
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // My test to make it fail
    // println!("After calling closure: {list:?}");
    // error[E0382]: borrow of moved value: `list`
}
```

## Moving Captured Values Out of Closures
The body of a closure can do any of the following: Move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. 

Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

- `FnOnce` applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.
- `FnMut` applies to closures that don’t move captured values out of their body but might mutate the captured values. These closures can be called more than once.
- `Fn` applies to closures that don’t move captured values out of their body and don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

```rust
impl<T> Option<T> {
    //  The F type is the type of the parameter named f, which is the closure we provide when calling unwrap_or_else.
    // The trait bound specified on the generic type F is FnOnce() -> T, which means F must be able to be called once, take no arguments, and return a T
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
    // Using FnOnce in the trait bound expresses the constraint that unwrap_or_else will not call f more than once. 
    // If the Option is None, f will be called once. (aka the closure)
    // Because all closures implement FnOnce, unwrap_or_else accepts all three kinds of closures and is as flexible as it can be.
}
```
Note: If what we want to do doesn’t require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the Fn traits. 
    - we could call unwrap_or_else(Vec::new) to get a new, empty vector if the value is None. The compiler automatically implements whichever of the Fn traits is applicable for a function definition.

Now let’s look at the standard library method sort_by_key, defined on slices, to see how that differs from unwrap_or_else and why sort_by_key uses FnMut instead of FnOnce for the trait bound.
- The closure gets one argument in the form of a reference to the current item in the slice being considered, and it returns a value of type K that can be ordered. This function is useful when you want to sort a slice by a particular attribute of each item. 

[sort_by_key (Function)](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key)
```console
pub fn sort_by_key<K, F>(&mut self, f: F)
where
    F: FnMut(&T) -> K,
    K: Ord,
```

```rust
// we have a list of Rectangle instances, and we use sort_by_key to order them by their width attribute from low to high.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. 
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // OUTPUT:
    // [
    // Rectangle {
    //     width: 3,
    //     height: 5,
    // },
    // Rectangle {
    //     width: 7,
    //     height: 12,
    // },
    // Rectangle {
    //     width: 10,
    //     height: 1,
    // },
    // ]
}

// If it were FnOnce we would get this error:
// error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
```
The closure |r| r.width doesn’t capture, mutate, or move anything out from its environment, so it meets the trait bound requirements.



## Other / Related Links
- [thread (Module)](https://doc.rust-lang.org/std/thread/index.html)
    - [join (Function)](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join)
- [slice (Primitive Type)](https://doc.rust-lang.org/std/primitive.slice.html)
    - [sort_by_key (Function)](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key)

## New Keywords Introduced

- move: Make a closure take ownership of all its captures.

