# Adding Functionality with Test-Driven Development

## Super Summary

```rust
// lib.rs - completed version with test
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


// Test 
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

        // Should return line that has 'duct' inside aka pro-duct-ive
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

## Adding Functionality with Test-Driven Development

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
This does another rudimentary step by step - so I follow the book but I just just added final result to the Super Summary and commented on it
</strong>
    </td>
  </tr>
</table>


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