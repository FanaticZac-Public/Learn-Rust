use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{search, search_case_insensitive};

// NOTE: THIS PROJECT WAS COPIED FROM CH12 SO I CAN DO THE IMPROVEMENTS FOR CHAPTER 13.

// src/main.rs
fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
    // fn build(args: &[String]) -> Result<Config, &'static str> {
    // Changed to factor in the iterator directly
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone();
        // let file_path = args[2].clone();

        // // Will return false if env var isn't set with .is_ok();
        // // let ignore_case = env::var("IGNORE_CASE").is_ok();

        // // mine because it wasn't actually checking value
        // let ignore_case = match env::var("IGNORE_CASE") {
        //     Ok(value) => value == "1",
        //     Err(_) => false,
        // };

        // Ok(Config {
        //     query,
        //     file_path,
        //     ignore_case, // Added for Working with Environment Variables section
        // })

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // Added for Working with Environment Variables section
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}
