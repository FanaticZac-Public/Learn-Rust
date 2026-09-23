// use std::io;

///// part 1 - Processing a Guess
// fn main() {
//     println!("Hello, world!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// My notes:
// In Rust:
// Variables are immutable by default. use mut for mutable
// :: referes to an associated function of a type such as String above
// &mut passes guess as a reference (no copy) - one of rusts major advantages are references being safe and easy.. more later
// like variables, references are not mutable by default. ** (chp4)
// read_line returns enum for the state - and expect handles our exception (chp6 for enum) but it's Ok or Err here
// -- rust requires this exception added -- won't compile otherwise - awesome

// semantically similar to c/c++ but 2 major Rust things here is the default immutability for variables and references and the forcing of exception checking
// Rustacean in training.

///// part 2 - Generating a Secret Number
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please input a number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// My notes:
// No rand in standard library - have to pull in a source crate
// -- had to 'rand = "0.8.5"' to toml - the cargo build or cargo check to load it
// data comes from Crates.io where people post their source Rust for others. (crate registry)
// Cargo.lock will prevent potential regression in crates from breaking your code. Should be saved in source control.
// cargo update - to force update of these dependencies to the latest
// "cargo doc --open" will build document provided by all dependencies locally and open in browser
// Creating the second guess variable is called shadowing - so you keep one type - of refence is of correct type
// the enter key will add the newline ot the imput 5\n so trim clears that too
// parse will work for any specificed return type set in the defintion
// weird loop syntax
// Note match expression is same as guess - but of different return enum. 

