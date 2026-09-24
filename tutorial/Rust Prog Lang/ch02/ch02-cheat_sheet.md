<style>
@import url("../notes.css");
</style>

# Chapter 02 - Programming Guessing Game

<div class="zac-note">
Ok - this chapter is largely introductory but for consistency sake I will just abbreviate the whole chapter and only add things of note.

For this chapter - Im just going to put in the final application and comment on segments.

Certainly check out the actual tutorial if interested in the broader discussion:
[Chapter 02 - Programming Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

</div>

Setting up the project with command:

```console
$ cargo new guessing_game
$ cd guessing_game
```

- This creates the package, and so generates the toml file
- TOML stands for Toms Obvious Minimal Language
  Default TOML:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.8.5" // this one gets added, then 'cargo build' to install.
```

- This let's you add modules from crate.io that is the public repository for rust that can support your program functionality.
- Cargo understands Semantic Versioning is a standard for writing version numbers.
  - The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.
    [Semantic Versioning](https://semver.org/)
  - Any version 0.9.0 or greater is not guaranteed to have the same API as what the following examples use.
  - Cargo will use only the versions of the dependencies you specified until you indicate otherwise
  - Rust creates the Cargo.lock file the first time you run cargo build
    - When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again (say to latest)
    - say that next week version 0.8.6 of the rand crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code.
      - This lets you have a reproducible build automatically.
    - In other words, your project will remain at 0.8.5 until you explicitly upgrade, thanks to the Cargo.lock file.
  - Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.
- 'cargo update' will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml and update up to 0.9.0 in this case.
- When you run cargo update - it will show you all newer version - but will only update up to 0.9.0.
  - If you want a later version, you must manually put that newer version in your toml file and run build again.
- More on this in chapter 14.
  Running 'cargo doc --open' will build documentation provided by all your dependencies locally and open it in your browser. - such as rand

---

Rust has a set of items defined in the standard library that it brings into the scope of every program automatically. This set is called the prelude, and you can see everything in it here:

[The Rust Prelude - aka std library modules](https://doc.rust-lang.org/std/prelude/index.html)

If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a 'use' statement. (std::io)

This is showing the final program from the chapter and I'll just going to add comments inside to explain it:

```rust
// Importing crates from the standard library api and crates.io.
use std::cmp::Ordering; // Standard Library - Compare - Ordering
use std::io;    // Standard Library - Input/Output
use rand::Rng; // From Crates.io - for random generation
// The Rng trait defines methods that random number generators implement

fn main() { // Main function - entry point for the program
    println!("Guess the number!"); // Typical print syntax

    // This generates a random from imported rand program
    // thread_rng() gives us a random number generator local to the current thread of execution and is seeded by the operating system.
    // then we call gen_range() method on the random number generator that is defined by the Rng trait that we brought into scope with the use rand::Rng; that takes a range expression as an argument and generates a random number in the range.
    // 1..=100 is the range expression in Rust (start..=end and is inclusive on the lower and upper bounds) aka 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop here just loops everything in the 'crab pincers'
    // I call {} curly braces, but to each their own
    loop {
        println!("Please input your guess.");

        // In rust - variables are immutable by default
        // you must manually indicate mutation
        // creates new String for holding user input
        // :: indicates that new is an associated function
        let mut guess = String::new(); //UTF-8 encoded bit of text.

        // Get Standard Input from Input/Output functionality of Standard Library
        // Read that line (from terminal) and assign to variable for mutable memory store
        // & marks variable or argument as reference, which lets your code access same memory (no copy into new memory).
        // .expect() will handle error and print error message - if fails.
        // the period (.) lets us daily chain our function calls
        // - output of stdin goes to read_line then goes to .expect
        // - based on the result
        io::stdin()
            .read_line(&mut guess) // up to the \n indicator added by enter key - aka press 5 and hit enter.
            .expect("Failed to read line");

        // 'match' is core keyword in Rust for comparing condition
        //  - (acts like switch from other languages)
        // https://doc.rust-lang.org/book/appendix-01-keywords.html
        // Our guess will either contain an Ok or an Error as result of the .expect() function above.
        // - So we check which and either return and unsigned int to guess or continue
        // - aka leaving this loop early and starting over.
        // .trim() clears whitespace and parse() does conversion from the String input to unsigned int into guess
        // parse doesn't need parameters, it can infer from return type : u32 =
        // Major note: We are 'shadowing' the previous 'let guess' command - in rust - this means this new variable is overshadowing the previous one (without having to create another variable), here with a new type of the same data - check chp3 cheatsheet for this.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch all for anything else
        };

        // 'crab pincers' escape the String literal to get the guess variable... obviously.... .. . ..
        println!("You guessed: {guess}");

        // Another 'match usage' comparing secret_number value to guess.
        // cmp returns one of these enum types that have associated functions
        // Depending on which it matches (Less, Greater, Equal),
        // the right hand side of the => for that match will execute aka printing out the related.
        // break will break out of the loop and destroy all humanity.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Summary

### Keywords

See proper definitions here:
[Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)

- let: Bind a variable.
- loop: Loop unconditionally.
- match: Match a value to patterns.
- break: Exit a loop immediately.
- continue: Continue to the next loop iteration.
- fn: Define a function or the function pointer type.
- mut: Denote mutability in references, raw pointers, or pattern bindings.
- use: Bring symbols into scope.

### Standard Library API Used

- [println (macro)](https://doc.rust-lang.org/std/macro.println.html)
- [String (struct)](https://doc.rust-lang.org/std/string/struct.String.html)
  - [trim (method of String)](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  - [parse (method of String)](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
- [io (Module)](https://doc.rust-lang.org/std/io/index.html)
  - [Stdin (Struct)](https://doc.rust-lang.org/std/io/struct.Stdin.html)
    - [read_line (Method of Stdin)](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line)
      - Read line returned Result - which is what we checked with 'match' keyword
    - [Result (Enum)](https://doc.rust-lang.org/std/result/enum.Result.html)
      - Returns 'Ok' or 'Err'
- [cmp (Module)](https://doc.rust-lang.org/std/cmp/index.html)
  - [Ordering (Enum)](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)

### Crates.io Library

- [rand (Crate)](https://crates.io/crates/rand)
  - [rng (Function)](https://docs.rs/rand/latest/rand/fn.rng.html)
    - [ThreadRng (Struct)](https://docs.rs/rand/latest/rand/rngs/struct.ThreadRng.html)

### Other Links

- [Semantic Versioning](https://semver.org/)
- [TOML Documentation](https://toml.io/en/)
