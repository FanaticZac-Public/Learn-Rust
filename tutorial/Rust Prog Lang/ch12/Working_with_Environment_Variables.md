# Working with Environment Variables

## Super Summary

````rust
// libs.rs
// added new functions and tests for case sensitivity
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
            search_case_insensitive(query, contents)
        );
    }
}
```rust
// main.rs
// using env variables to call either search or search insensitive

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

// Adjusted code in Config to handle env variables with env::var()
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
````

I adjusted the code from the lesson because it was checking whether variable existed and not what it was

```rust
// mine because it wasn't actually checking value
let ignore_case = match env::var("IGNORE_CASE") {
    Ok(value) => value == "1",
    Err(_) => false,
};
```

My testing based on changing env variables on linux

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
This was really annoying me that it wasn't checking the actual value when i changed the env variable in linux. 
My change below checks via match, (will also default to case sensitive with no value set) but actually works now.</strong>
    </td>
  </tr>
</table>

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
