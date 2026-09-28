// Example 1 - Command arguments setup

// use std::env; // bring parent into scope
// // can call child function env::args()

// fn main() {
//     // collect function creates collections, we specify we want a vector of strings.
//     let args: Vec<String> = env::args().collect();
//     dbg!(args); // print the vector using the debug macro

//     // OUTPUT
//     // [src/main.rs:9:5] args = [
//     // "target/debug/minigrep",
//     // ]
// }

// Example 2 - Saving the variables

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let file_path = &args[2];

//     println!("Searching for {query}");
//     println!("In file {file_path}");
// }

// Example 3 - Reading the poem from file with args

// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let file_path = &args[2];

//     println!("In file {file_path}");

//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// Example 4 - Refactoring to Improve Modularity and Error Handling
// Full solution for end of section

use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{search, search_case_insensitive};

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
    ignore_case: bool, // Added for Working with Environment Variables section
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // Will return false if env var isn't set with .is_ok();
        // let ignore_case = env::var("IGNORE_CASE").is_ok();

        // mine because it wasn't actually checking value
        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => value == "1",
            Err(_) => false,
        };

        Ok(Config {
            query,
            file_path,
            ignore_case, // Added for Working with Environment Variables section
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Added for Working with Environment Variables section
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
