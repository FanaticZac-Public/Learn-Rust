# To panic! or Not to panic!

## Super Summarize

```rust
// Mostly this part is basics... but there are some examples of stranger use cases

// It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.

//-------------- Informative rather than systemic --------------
// To check hardcoded - inform programmer
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

//-------------- Improvement on guessing game error handling --------------
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// that's it - not much of value otherwise in this chapter.
```

## To panic! or Not to panic!

If unrecoverable - like security risk or some other serious risk - panic

If recoverable - aka you want code to make a decision - return Result enum

### Examples, Prototype Code, and Tests

Small concepts or code examples - can panic

_... i don't want to copy everything here - i get it - just noting down Rust specific mentions_

unwrap and expect methods are very handy when you’re prototyping and you’re not yet ready to decide how to handle errors.

- concise and minimally effective - indicate where to add robustness if required

### When You Have More Information Than the Compiler

It would also be appropriate to call expect when you have some other logic that ensures that the Result will have an Ok value

```rust
  use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
```
- just informs dev that it should be valid - checks it if not


### Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
- The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
- Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
- There’s not a good way to encode this information in the types you use. We’ll work through an example of what we mean in “Encoding States and Behavior as Types” in Chapter 18.

If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so that the user of the library can decide what they want to do in that case. 

However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so that they can fix it during development. 

Similarly, panic! is often appropriate if you’re calling external code that is out of your control and returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.

When your code performs an operation that could put a user at risk if it’s called using invalid values, your code should verify the values are valid first and panic if the values aren’t valid. This is mostly for safety reasons: Attempting to operate on invalid data can expose your code to vulnerabilities. 

This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: Trying to access memory that doesn’t belong to the current data structure is a common security problem.

Functions often have contracts: Their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug, and it’s not a kind of error you want the calling code to have to explicitly handle.

 In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

 However, having lots of error checks in all of your functions would be verbose and annoying. Fortunately, you can use Rust’s type system (and thus the type checking done by the compiler) to do many of the checks for you. If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured that you have a valid value. For example, if you have a type rather than an Option, your program expects to have something rather than nothing. Your code then doesn’t have to handle two cases for the Some and None variants: It will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. Another example is using an unsigned integer type such as u32, which ensures that the parameter is never negative.

### Custom Types for Validation
Let’s take the idea of using Rust’s type system to ensure that we have a valid value one step further and look at creating a custom type for validation. 

Recall the guessing game in Chapter 2 in which our code asked the user to guess a number between 1 and 100. We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive. In this case, the consequences were not very dire: Our output of “Too high” or “Too low” would still be correct. But it would be a useful enhancement to guide the user toward valid guesses and have different behavior when the user guesses a number that’s out of range versus when the user types, for example, letters instead.

One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range, like so:


```rust
    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }
```

The if expression checks whether our value is out of range, tells the user about the problem, and calls continue to start the next iteration of the loop and ask for another guess. After the if expression, we can proceed with the comparisons between guess and the secret number knowing that guess is between 1 and 100.

However, this is not an ideal solution: If it were absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).

Instead, we can make a new type in a dedicated module and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive. Listing 9-13 shows one way to define a Guess type that will only create an instance of Guess if the new function receives a value between 1 and 100.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

- Note that this code in src/guessing_game.rs depends on adding a module declaration mod guessing_game; in src/lib.rs that we haven’t shown here. 
- ithin this new module’s file, we define a struct named Guess that has a field named value that holds an i32. This is where the number will be stored.
- Then, we implement an associated function named new on Guess that creates instances of Guess values.

 The code in the body of the new function tests value to make sure it’s between 1 and 100. If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on.
- *this is what is meant by contract*

 This public method is necessary because the value field of the Guess struct is private.
 - *struct is public but values are not (you can make your own structs - not see Guess's)*

 - Code outside the guessing_game module must use the Guess::new function to create an instance of Guess, thereby ensuring that there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.

A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than an i32 and wouldn’t need to do any additional checks in its body.

