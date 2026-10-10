# Test Organization

## Super Summary
```rust
//-------------- Unit Tests --------------
// ---- The tests Module and #[cfg(test)] ----

// - Unit tests are located in the same files (e.g. lib.rs) as the functions they test
// - cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option. 

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// - private functions can be tested directly (not true in other languages)
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)] // < run on 'cargo test', not 'cargo build'
mod tests {
    use super::*; // < brings all outer functions in file into this mod

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // private test function
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

}


//-------------- Integration Tests --------------
// Only library crates expose functions that other crates can use; 
// binary crates are meant to be run on their own

// ---- The tests Directory ----

// project_name
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     └── integration_test.rs

// (Cargo looks for Integration tests in the 'tests' folder
// (Cargo will compile each of the files in 'tests' as an individual crate)

// Example of tests/integration_test.rs
use adder::add_two; // <- bring into scope

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

// #[cfg(test)] is not required here
// To run specific test: $ cargo test --test integration_test

// ---- Submodules in Integration Tests ----
// Because files in 'tests' compile as an individual crate,
// if you want to make more files in the tests directory to help organize them (by functionality),  
// you can't create helper functions with the same path syntax


// Example to extract them into a common module /common/mod.rs:
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     ├── common
//     │   └── mod.rs
//     └── integration_test.rs

// (see ch07 for more information on alt file paths where mod.rs files are discussed)
// - Basically creating a folder with the name and a file with name mod.rs like so 'tests/common/mod.rs' is equal to 'tests/common.rs'
// - and files in subdirectories of the tests directory won't get compiled as separate crates or have sections in the test output.
// In tests/common/mod.rs - you can create whatever assistive function you want for the tests.

// Then in: tests/integration_test.rs
use adder::add_two;

mod common; // import module common

#[test]
fn it_adds_two() {
    common::setup(); // Run whatever functions there.

    let result = add_two(2);
    assert_eq!(result, 4);
}

```

## Original Notes - Test Organization - Original Notes


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


 