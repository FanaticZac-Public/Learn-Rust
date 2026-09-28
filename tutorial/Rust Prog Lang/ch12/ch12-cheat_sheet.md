<style>
@import url("../notes.css");
</style>

# Chapter 12 - An I/O Project: Building a Command Line Program - Cheat Sheet - Quick Reference

Topics Covered:

- Command Line Arguments
- File operations
- Error handling
- Test organization
  - Unit Tests
  - Integration tests
- Test Driven Development
- Redirect output to stdout vs stderr

Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools

This chapter recreates grep command
- grep: (globally search a regular expression and print).

One Rust community member, Andrew Gallant, has already created a fully featured, very fast version of grep, called ripgrep

## Accepting Command Line Arguments

We want to make command line accept 2 arguments for usage like this

```console
$ cargo run -- searchstring example-filename.txt

// two hyphens indicate the following arguments are for our program rather than for cargo
// We want:
// - a string to search for
// - a path to a file to search in
```

There are existing libraries on crates.io to help with writing a program but for this exercise we implement this ourselves.

### Reading the Argument Values

To read the values of command line arguments we pass to it, we’ll need the [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) function provided in Rust’s standard library.

- returns an iterator of the command line arguments
  - Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, which contains all the elements the iterator produces.

In Rust, you very rarely need to annotate types, however collect is one function you do because it can't infer the kind of collection you want.

```rust
// src/main.rs

use std::env; // bring parent into scope
// can call child function env::args()

fn main() {
    // collect function creates collections, we specify we want a vector of strings.
    let args: Vec<String> = env::args().collect();
    dbg!(args); // print the vector using the debug macro
}
```

Note that first result is the compiled binary itself

```console
    // OUTPUT from 'cargo run'
    [src/main.rs:9:5] args = [
    "target/debug/minigrep",
    ]

    $ cargo run -- needle haystack
    args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
    ]
```

#### SIDE NOTE - The args Function and Invalid Unicode

- std::env::args will panic if any argument contains invalid Unicode
- std::env::args_os instead to accept arguments containing invalid Unicode
  - returns an iterator that produces OsString values instead of String values
    - OsString values differ per platform and are more complex to work with than String values.

### Saving the Argument Values in Variables

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // COMMAND: cargo run -- test sample.txt
    // OUTPUT:
    // Searching for test
    // In file sample.txt
}
```

<div class="zac-note">
TODO - COULD CONDENSE THE PREVIOUS EXAMPLES INTO THE BELOW ONE
</div>
## Reading a File

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    // poem.txt in root folder
    // COMMAND: cargo run -- the poem.txt
    // OUTPUTS: full poem
}
```

## Refactoring to Improve Modularity and Error Handling

There are 4 problems/improvements that can be made:

1. Separate out the 2 tasks, reading arguments and reading files
2. Group configuration variables, and content variables
3. Using expect to generate more robust error handling
4. We can add error handling regarding the arguments (#) as well

### Separating Concerns in Binary Projects

- Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in the main function.
- When the command line parsing logic starts getting complicated, extract it from the main function into other functions or types.

The responsibilities of main.js should be limited to the following:

1. Calling the command line parsing logic with the argument values
2. Setting up any other configuration
3. Calling a run function in lib.rs
4. Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program and lib.rs handles all the logic of the task at hand.

A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.

<div class="zac-note">
The tutorial builds this program section by section - but this is rudimentary so i'll just put final result following my summary of events
</div>

1. Extracting the Argument Parser
   - separation of concerns
2. Grouping Configuration Values
   - Added struct over tuple to keep related arg variables together and aptly named for maintainers
   - used .clone() on the arguments to copy to obey ownership rules.
     - Less efficient but in this case it's not a look and pretty lightweight program - and avoid managing lifetimes for this exercise
3. Creating a Constructor for Config
   - Created Config structure instead with it's own impl (implemented) method new() aka constructor to be more more idiomatic.
4. Fixing the Error Handling
   - Avoided `index out of bounds` error for the arguments by doing a check of the incoming arguments to match 'expected' and added error message to that effect.
5. Returning a Result Instead of Calling panic!
   - Changed method name from 'new' to build, because devs often think 'new' can't fail, and use Result enum, and use Err to prompt a more descriptive error there as well.
6. Calling Config::build and Handling Errors
   - We shift the program away from exiting with panic to using nonzero error code with standard library `process` module.
     - We also use newly introduced function unwrap_or_else() on Result to manage the custom non-panic! error message.
       - Returns value on Ok, but on Err returns anonymous function that prints error and runs `process::exit(1);`
     - `process::exit(1);` will stop program and return integer as status code.
       - Returns less output on exit, cleaner result.
7. Extracting Logic from main
   - Separated out the file access logic from main as well
8. Returning Errors from run
   - Instead of allowing program to panic! from file errors, we return Result enum from that as well - keeping error handling all in main with return type `Result<(), Box<dyn Error>>`
     - Box<dyn Error> trait object is introduced via `use std::error::Error` (Trait Objects in ch18) but means the function will return a type that implements the Error trait, so we don’t have to specify what particular type Error will be (handles various error cases)
   - Moved `expect` to `?` operator to return the error value from current function to handle.
   - Run function returns Ok in success case.
     - `Ok(())` is the idiomatic way to indicate that we’re calling run for its side effects only without returning a value
9. Handling Errors Returned from run in main
   - Used `if let` to check wether run returns an Err as we no longer return a value in the `Ok(())` so no unwrap in unwrap_or_else() required. We only care about detecting the error.
10. Splitting Code into a Library Crate
    - Define search function in src/lib.rs with a body that calls unimplemented! macro.
    - `pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { unimplemented!(); }`
    - Then import `use minigrep::search;` back into main.rs
    - And use it in for loop to search each line and print lines that match the search term `query` from $contents (aka poem).
      - implementation pending

### Final solution for Refactoring section

```rust
// src/lib.rs

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!(); // placeholder for ... you know.
}
```

```rust
// src/main.rs

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

// src/main.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

## Adding Functionality with Test-Driven Development

<div class="zac-note>
Up until now in the section i was careful to make this cheatsheet almost publish worthy. But I just woke up and am reading and refreshing TDD... (and there's 3 more sections in this chapter) my focus on the subject is high but not on the cheat sheet. - consider the following section to required a review and super summarization after for cheat sheet worthiness.
</div>
Now that we have the search logic in src/lib.rs separate from the main function, it’s much easier to write tests for the core functionality of our code. We can call functions directly with various arguments and check return values without having to call our binary from the command line.

In this section, we’ll add the searching logic to the minigrep program using the test-driven development (TDD) process with the following steps:

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

### Writing a Failing Test

- TDD can help drive code design.
- Writing the test before you write the code that makes the test pass helps maintain high test coverage throughout the process.
- We'll do this with our search function for the mini-grep
  - returning `vec![]` instead of `!unimplemented`

#### Lifetime talk

- We define an explicit lifetime 'a in the signature of search and use that lifetime with the contents argument and the return value because we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
  - we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.
  - (zac - hard to get head around but simplest when i phrase it it as "this syntax forces these variables to variables share a lifetime" - aka incoming parameter and return type - in this case)
  - The data referenced by a slice needs to be valid for the reference to be valid
  - if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.
  - error[E0106]: missing lifetime specifier (without it)
    - Note that the extra help text suggests specifying the same lifetime parameter for all the parameters and the output type, which is incorrect!
    - We just want it for the contents as that's what will be returned and the compiler will want to know which lifetimes to match, query won't be returned and can be dropped (memory).
- Rust can’t know which of the two parameters we need for the output, so we need to tell it explicitly.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // backslash tells rust not to place '\n'
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // We assert returned like is the one that has 'duct' inside
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

- query and contents mimics our command args
- backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal
- We assert that the value returned from the search function contains only the line we expect.
- In accordance with TDD principles, we’ll take a small step of adding just enough code to get the test to not panic when calling the function by defining the search function to always return an empty vector

### Writing Code to Pass the Test

Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:

1. Iterate through each line of the contents.
2. Check whether the line contains our query string.
3. If it does, add it to the list of values we’re returning.
4. If it doesn’t, do nothing.
5. Return the list of results that match.

<div class="zac-note"> 
This does another rudimentary step by step - so ill follow the book but just add final result here and comment on it - whatever is worth remembering or unique to rust
</div>

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // returns iterator
    for line in contents.lines() {
        // contains() is method of iterator
        if line.contains(query) {
            results.push(line);
        }
    }

    results
    // Test passes now
}
```

Now that test passed with out basic implementation we could improve the function (ch13) or run the whole program and see result.

It works to query 'frog' and 'body'.

## Working with Environment Variables

We’ll improve the minigrep binary by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable. 

 We could make this feature a command line option and require that users enter it each time they want it to apply, but by instead making it an environment variable, we allow our users to set the environment variable once and have all their searches be case insensitive in that terminal session.

### Writing a Failing Test for Case-Insensitive Search

We’ll continue to follow the TDD process, so the first step is again to write a failing test.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() { // changed name of test
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape."; // Added here to test

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // new function added
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```
We use the old search function but add another function for case insensitivity
```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase(); // added (string now)
    let mut results = Vec::new();

    for line in contents.lines() {
        // to_lowercase() added
        if line.to_lowercase().contains(&query) { 
            results.push(line);
        }
    }

    results

    // test tests::case_insensitive ... ok
    // test tests::case_sensitive ... ok
}
```
- While to_lowercase will handle basic Unicode, it won’t be 100 percent accurate. If we were writing a real application, we’d want to do a bit more work here, but this section is about environment variables, not Unicode, so we’ll leave it at that here.
- to_lowercase creates new String, new data
- say the query is "rUsT", as an example: That string slice doesn’t contain a lowercase u or t for us to use, so we have to allocate a new String containing "rust". 
- We add to_lowercase to each like to check match

Now we add a option to allow this in our application configuration struct

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, // added
}
```

Adjusting run function to check for the ignore_case bool.

```rust
use minigrep::{search, search_case_insensitive};

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    dbg!(config.ignore_case);
    let results = if config.ignore_case { // check
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```
Finally, we need to check for the environment variable
- The functions for working with environment variables are in the env module in the standard library, which is already in scope at the top of src/main.rs. 
- We’ll use the var function from the env module to check to see if any value has been set for an environment variable named IGNORE_CASE

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Will return false if env var isn't set with .is_ok();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```
<div class="zac-note">

This was really annoying me that it wasn't checking the actual value when i changed the env variable in linux. 
My change below checks via match, (will also default to case sensitive with no value set) but actually works now.
```rust
// mine because it wasn't actually checking value
let ignore_case = match env::var("IGNORE_CASE") {
    Ok(value) => value == "1",
    Err(_) => false,
};
```

```console
cargo run -- to poem.txt

// with export IGNORE_CASE=1

Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!

// wih export IGNORE_CASE=0

Are you nobody, too?
How dreary to be somebody!

// with unset IGNORE_CASE

Are you nobody, too?
How dreary to be somebody!
```

</div>

## Redirecting Errors to Standard Error

2 Kinds of terminal output:
1. stdout (Standard Output)
    - println!() uses this
2. sterr (Standard Error)

```console
// Directing standard output (stout) to file
$ cargo run > output.txt

// This will not direct stderr to file.
```

### Printing Errors to Standard Error
```rust
// printing to stderr instead in Rust
     eprintln!("Problem parsing arguments: {err}");
```
```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

## Other / Related Links

- [env (Module)](https://doc.rust-lang.org/std/env/index.html)
  - [Args (Struct)](https://doc.rust-lang.org/std/env/fn.args.html)
- [Iterator (Trait)](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  - [collect (method)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
- [fs (Module)](https://doc.rust-lang.org/std/fs/index.html)
  - [read_to_string (Function)](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
- [process (Module)](https://doc.rust-lang.org/std/process/index.html)
  - [exit (Function)](https://doc.rust-lang.org/std/process/fn.exit.html)
- [Result (Enum)](https://doc.rust-lang.org/std/result/enum.Result.html)
  - [unwrap_or_else (Function)](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else)
- [error (Module)](https://doc.rust-lang.org/std/error/index.html)
  - [Error (Trait)](https://doc.rust-lang.org/std/error/trait.Error.html)

## New Keywords Introduced

- dyn: Dynamic dispatch to a trait object.
