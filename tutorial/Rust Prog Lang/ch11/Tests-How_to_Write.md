# How to Write Tests

## Super Summary
```console
// ------ Running tests  -------
// In console to run tests you write:
$ cargo test           
$ cargo test --lib      // or
```
```rust
// The bodies of test functions typically perform these three actions:
// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert that the results are what you expect.


//-------------- Structuring Test Functions  --------------
// function to test (likely within a lib.rs file)
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Test function (typically located at bottom of same file)
#[cfg(test)] // <- Marks a test module 
mod tests {
    use super::*;

    #[test]
    fn it_works() { // <- Marks a test function
        let result = add(2, 2);
        assert_eq!(result, 4); // < - Test condition (pass/fail)
    }
}

// ------ Example output:  -------
// running 1 test
// test tests::it_works ... ok
// 
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


//-------------- Testing Equality with assert_eq! and assert_ne!   --------------
// 3 options for test assertions
- assert!(result = 4); // Less descriptive message
- assert_eq!(4, 4); // true
- assert_ne!(4, 4); // false
// For structs and enums that you define yourself:
// - need to implement PartialEq to assert equality of those types.
// - need to implement Debug to print the values when the assertion fails.

// Note: In Rust, the parameters to equality assertion functions are called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.


//-------------- Adding Custom Failure Messages   --------------
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`" // this line added
    );
}
// ------ Example output:  -------
// running 1 test
// test tests::greeting_contains_name ... FAILED
// 
// failures:
// 
// ---- tests::greeting_contains_name stdout ----
// 
// thread 'tests::greeting_contains_name' (6021333) panicked at src/lib.rs:12:9:
// Greeting did not contain name, value was `Hello!`


//-------------- Testing for something that should panic!   --------------
// You can check where 
#[should_panic] // Checks if panic but could be multiple reasons
#[should_panic(expected = "less than or equal to 100")] // checks for substring

// Example
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

// or 
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}


//-------------- Using Result<T, E> in Tests   --------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}


```

## Original Notes - How to Write Tests - Original Notes

The bodies of test functions typically perform these three actions:

- Set up any needed data or state.
- Run the code you want to test.
- Assert that the results are what you expect.

## Run tests
```console
// Run test
$ cargo test

// for library crate
$ cargo test --lib
```

## Writing Tests

To change a function into a test function, add `#[test]` on the line before fn

Asserting a result has 3 syntax options:

```rust
- assert!(result = 4); // Less descriptive message
- assert_eq!(result, 4);
- Assert_ne!(3)
```

Note: 
These macros print their arguments using debug formatting
- the values being compared must implement the PartialEq and Debug traits. 
- All primitive types and most of the standard library types implement these traits.
- For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types.
- You’ll also need to implement Debug to print the values when the assertion fails.

Note: In Rust, the parameters to equality assertion functions are called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.

```rust
assert_eq!(4, result)
// which would result in the same failure message that displays assertion `left == right` failed.
```

Testing for something that should panic!
```rust
- #[should_panic] // Checks if panic but could be multiple reasons
- #[should_panic(expected = "less than or equal to 100")] // checks for substring
```

Two examples for showing syntax and Attributes options

```rust
// Example Code 1 - Example of function you want to test
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Example Test 1
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        // variations of handling result - choose 1 or the 4
        // Note - second param from custom error (result = 4, "custom msg")

        // Regular assert - Less descriptive message on failure
        assert!(result = 4); // or e.g. result.contains("Carol"), any bool

        // Assert equal
        assert_eq!(result, 4);

        // Assert not equal
        Assert_ne!(3)

        // Using Result<T, E>
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // ---------------------------------------------------------

    // Example Code 2 - Example of function you want to test

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {value}."
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {value}."
                );
            }

            Guess { value }
        }
    }

    // Example Test 2 - should_panic and expect syntax
    #[cfg(test)]
        mod tests {
        use super::*;

        #[test]
        // Choose 1 of 2 --------------
        #[should_panic]         // Test passes if panics - but could panic for multi-reasons
        #[should_panic(expected = "less than or equal to 100")]         // Pass with expected error text
        // -----------------
        fn greater_than_100() {
            Guess::new(200);
        }
    }
}
```

Example output

```console
$ cargo test
    running 1 test
    test tests::it_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Doc-tests adder
```
- pass/fail is self explanatory
- ignore (some tests can be marked ignore - see further down)
- measured (for performance - needs setup): [Benchmark Tests (Performance) ](https://doc.rust-lang.org/unstable-book/library-features/test.html)
- `Doc-tests adder` is mentioned in ch14
- filtered: pass an argument to the cargo test command to run only tests whose name matches a string
