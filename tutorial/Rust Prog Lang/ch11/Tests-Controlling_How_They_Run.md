# Controlling How Tests Are Run

## Super Summary
```console
// Run all tests
$ cargo test     
$ cargo test --lib      // or

// Get test options
$ cargo test --help

// Get test options for using the '--' separator
$ cargo test -- --help

//-------------- Run sequentially (or fewer threads) --------------
// Test run parallel by default - should be interdependent by default.
// However if say testing file operations and want sequential:
// Run tests single threaded
$ cargo test -- --test-threads=1


//-------------- Show the normal println output (error output always shown) --------------
// Tests only print the println! macro in the fn if tests fail. If you want to see the output: 
$ cargo test -- --show-output


//-------------- Filter Tests to Run --------------
// To run a subset of the tests - aka filter out others:
// To run one test by name - will show others as filtered out on result
$ cargo test example_test_name

// To run multiple by filter - call using shared substring of the test name 
// e.g. (say you have 3 tests that have 'add' as part of test name)
$ cargo test add


//-------------- Running tests marked ignore --------------
// Run only ignored
$ cargo test -- --ignored

// Run all including ignored
$ cargo test -- --include-ignored
```

```rust
//-------------- Ignoring Tests Unless Specifically Requested --------------
#[test]
#[ignore]  // <-
fn expensive_test() {
    // code that takes an hour to run
}
```

```console
// Example output relating to the above

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



## Original Notes - Controlling How Tests Are Run - Original Notes

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