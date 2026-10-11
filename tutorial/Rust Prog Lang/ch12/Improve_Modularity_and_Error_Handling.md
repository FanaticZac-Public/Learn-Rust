# Refactoring to Improve Modularity and Error Handling

## Super Summary

```rust
//-------------- Separating Concerns in Binary Projects --------------
// Includes:
// - Separating Concerns in Binary Projects
// - - Extracting the Argument Parser
// - - Grouping Configuration Values
// - - Creating a Constructor for Config
// - Fixing the Error Handling
// - - Improving the Error Message
// - - Returning a Result Instead of Calling panic!
// - - Calling Config::build and Handling Errors
// - - Extracting Logic from main
// - - Returning Errors from run
// - - Handling Errors Returned from run in main
// - Splitting Code into a Library Crate

// src/main.rs

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search; // moved search to lib.rs

// src/main.rs
fn main() {
    let args: Vec<String> = env::args().collect();

    // Error handling here with proper error messages
    // Useful logic moved out to other functions and structures

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

// Config holds argument data
struct Config {
    query: String,
    file_path: String,
}

// moves build logic and value checking to method of Config
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone() isn't ideal because it's copy rather than ref
        // - but used for this chapter to be simpler
        let query = args[1].clone(); 
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// anonymous assistant function for reading file lines
// - moves out of main to keep main 
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // calls search function from 'use minigrep::search;' in lib.rs
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```
```rust
// lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // setup signature and mark unimplemented just to import in main.rs
    unimplemented!();
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
The tutorial builds this program section by section - but this is rudimentary so i'll just put final result following my summary of events
</strong>
    </td>
  </tr>
</table>




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