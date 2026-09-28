<style>
@import url("../notes.css");
</style>


# Improving Our I/O Project

Let’s look at how iterators can improve our implementation of the Config::build function and the search function.

## Removing a clone Using an Iterator
Previously, we added code that took a slice of String values and created an instance of the Config struct by indexing into the slice and cloning the values, allowing the Config struct to own those values. 

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```
We needed clone here because we have a slice with String elements in the parameter args, but the build function doesn’t own args. 

To return ownership of a Config instance, we had to clone the values from the query and file_path fields of Config so that the Config instance can own its values.

With our new knowledge about iterators, we can change the build function to take ownership of an iterator as its argument instead of borrowing a slice. 

We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations. 

This will clarify what the Config::build function is doing because the iterator will access the values.

Once Config::build takes ownership of the iterator and stops using indexing operations that borrow, we can move the String values from the iterator into Config rather than calling clone and making a new allocation.

## Using the Returned Iterator Directly

```rust
   let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // TO THIS INSTEAD
    // let args: Vec<String> = env::args().collect();
                            // env::args() instead
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
```

The env::args function returns an iterator

Rather than collecting the iterator values into a vector and then passing a slice to Config::build, now we’re passing ownership of the iterator returned from env::args to Config::build directly.

Next, we need to update the definition of Config::build to:
```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {

// to this
impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--
```

We’ve updated the signature of the Config::build function so that the parameter args has a generic type with the trait bounds impl Iterator<Item = String> instead of &[String]. 

This usage of the impl Trait syntax means that args can be any type that implements the Iterator trait and returns String items.

Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we can add the mut keyword into the specification of the args parameter to make it mutable.

## Using Iterator Trait Methods

Next, we’ll fix the body of Config::build. Because args implements the Iterator trait, we know we can call the next method on it! 

```rust
impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        // OLD -------------------------
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
        // OLD END -------------------------

        // to this
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
```

Remember that the first value in the return value of env::args is the name of the program. We want to ignore that and get to the next value, so first we call next and do nothing with the return value. 

Then, we call next to get the value we want to put in the query field of Config. If next returns Some, we use a match to extract the value.If it returns None, it means not enough arguments were given, and we return early with an Err value. We do the same thing for the file_path value.

## Clarifying Code with Iterator Adapters

We can also take advantage of iterators in the search function in our I/O project, which is reproduced here

We can write this code in a more concise way using iterator adapter methods.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// to this:
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Doing so also lets us avoid having a mutable intermediate results vector. The functional programming style prefers to minimize the amount of mutable state to make code clearer. Removing the mutable state might enable a future enhancement to make searching happen in parallel because we wouldn’t have to manage concurrent access to the results vector. 

Recall that the purpose of the search function is to return all lines in contents that contain the query. Similar to the filter example in Listing 13-16, this code uses the filter adapter to keep only the lines for which line.contains(query) returns true. We then collect the matching lines into another vector with collect.

Much simpler! Feel free to make the same change to use iterator methods in the search_case_insensitive function as well...
<div class="zac-note">
Ok i will! 

```rust
// pub fn search_case_insensitive<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
```
</div>

For a further improvement, return an iterator from the search function by removing the call to collect and changing the return type to impl Iterator<Item = &'a str> so that the function becomes an iterator adapter.
<div class="zac-note">
Fine... 

Reminder: Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

map was the example given by the previous sub-chapter. 
Looked up the signature on the API:

impl: Implement inherent or trait functionality.

ok ok. This was easy enough to add in the return type and remove collect(). I got tripped up by the original rule affecting this section - even though they are the same return type - i can not use this function because it expects the same return type... 
```rust
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // it says type mismatch even through return types are the same. Because the iterator type is not technically the same.

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
    }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}");
    }
}
```




</div>

<div class="zac-note">Tests still work - code looks fine</div>

Note that you’ll also need to update the tests! Search through a large file using your minigrep tool before and after making this change to observe the difference in behavior. Before this change, the program won’t print any results until it has collected all of the results, but after the change, the results will be printed as each matching line is found because the for loop in the run function is able to take advantage of the laziness of the iterator.