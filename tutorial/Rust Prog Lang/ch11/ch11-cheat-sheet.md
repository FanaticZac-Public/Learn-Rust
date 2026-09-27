# Chapter 09 - Writing Automated Tests - Cheat Sheet - Quick Reference

[Writing Automatic Tests - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-00-testing.html)

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

## Controlling how tests are run
```console
// Run all tests
$ cargo test

// Get test options
$ cargo test --help

// Get test options for using the '--' separator
cargo test -- --help

```
### thread control
Test run parallel by default - should be interdependent by default.

However if say testing file operations and want sequential:
```console
// Run tests single threaded
cargo test -- --test-threads=1
```
Tests only print the println! macro in the fn if tests fail. If you want to see the output: 
```console
cargo test -- --show-output
```

### filter
To run a subset of the tests - aka filter out others:
```console
// To run one test by name - will show others as filtered out on result
$ cargo test example_test_name

// To run multi - call using shared substring of the test name 
// e.g. (say you have 3 tests with 'add' as part of test name)
cargo test add
```

### Ignored
To mark some tests with #[ignore] attribute 
```rust
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
```
Ignore options
```console
// Run only ignored
$ cargo test -- --ignored

// Run all including ignored
$ cargo test -- --include-ignored
```

## Test Organization
Two main categories: 
- `Unit tests` are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
- `Integration tests` are entirely external to your library and use your code in the same way any other external code would
    - using only the public interface and potentially exercising multiple modules per test.

### Unit Tests
Test functions should be in each file with the code that they’re testing. 
- ` #[cfg(test)]` The convention is to create a module named tests in each file to contain the test functions and to annotate the module with `cfg(test)` to inform cargo to only run with `cargo test` command.
    - cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option (in this case test) 
- Rust lets you test private functions (some languages don't)

### Integration Tests
In Rust, integration tests are entirely external to your library and purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated.

To create integration tests, you first need a tests directory.

```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```
Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test. 
```rust
// tests/integration_test.rs
use adder::add_two; // import required

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```
The three sections of output include the unit tests, the integration test, and the doc tests. 
```console
$ cargo test
-- snip --

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Note that if any test in a section fails, the following sections will not be run, in order: 
- unit tests (shows as `internal` in console output above)
- integration tests (shows as `tests/integration_test.rs` in console output above)
- doc tests (Not discussed in this chapter)

```
// Run a particular integration test by specifying function’s name 
$ cargo test it_adds_two

// To run all the tests in a particular integration test file only
$ cargo test --test integration_test
```

#### Submodules in Integration Tests

As integration tests grow, you can make more files in the tests directory to help organize them (by functionality, etc.). 

Each file in the tests directory is compiled as its own separate crate
- which is useful for creating separate scopes to more closely imitate the way end users will be using your crate. 
- However, this means files in the tests directory don’t share the same behavior as files in src do
    - most noticeable when you have a set of helper functions to use in multiple integration test files, and you try to follow the steps in the “Separating Modules into Different Files” section of [Chapter 7](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html) to extract them into a common module. 

For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files:
```rust
// tests/common.rs

pub fn setup() {
    // setup code specific to your library's tests would go here
}

// CONSOLE OUTPUT
    // running 1 test
    // test tests::internal ... ok

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    //      Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)
```
Having common appear in the test results with running 0 tests displayed for it is not what we wanted. 

Rust has a convention for this, to create a `tests/common/mod.rs`:
```console
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

```
Naming the file this way tells Rust not to treat the common module as an integration test file. 
- When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear.
- Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.
- same path rules from [Alternate File Paths - Ch7](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#alternate-file-paths)

To setup test to work with tests/common/mods.rs in test file: 
```rust
// tests/integration_test.rs

use adder::add_two;

mod common; // import common from tests/common/mod.rs

#[test]
fn it_adds_two() {
    common::setup(); // call common.setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
```

#### Integration Tests for Binary Crates
Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

## Other Links Related

[Attributes in Rust](https://doc.rust-lang.org/reference/attributes.html)

[Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

[Testing Chapter of 'The rustc book'](https://doc.rust-lang.org/rustc/tests/index.html)


