# Accepting Command Line Arguments

## Super Summary

```console
// two hyphens to indicate the following arguments are for our program rather than for cargo
// What we want to call:
$ cargo run -- searchstring example-filename.txt
```

```rust
// Note: std::env::args will panic if any argument contains invalid Unicode
// std::env::args_os to accept arguments containing invalid Unicode (adds complexity)
use std::env;

fn main() {
    // In Rust, you very rarely need to annotate types, however collect is one function you do because it can't infer the kind of collection you want.
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // COMMAND: cargo run -- test sample.txt
    // OUTPUT:
    // Searching for test
    // In file sample.txt
}
```

```console
$ cargo run -- needle haystack
args = [
"target/debug/minigrep",
"needle",
"haystack",
]
```

## Accepting Command Line Arguments

We want to make command line accept 2 arguments for usage like this

```console
$ cargo run -- searchstring example-filename.txt

// two hyphens indicate the following arguments are for our program rather than for cargo
// We want:
// - a string to search for
// - a path to a file to search in
```

There are existing libraries on crates.io to help with writing a program but for this exercise we implement this ourselves.

### Reading the Argument Values

To read the values of command line arguments we pass to it, we’ll need the [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) function provided in Rust’s standard library.

- returns an iterator of the command line arguments
  - Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, which contains all the elements the iterator produces.

In Rust, you very rarely need to annotate types, however collect is one function you do because it can't infer the kind of collection you want.

```rust
// src/main.rs

use std::env; // bring parent into scope
// can call child function env::args()

fn main() {
    // collect function creates collections, we specify we want a vector of strings.
    let args: Vec<String> = env::args().collect();
    dbg!(args); // print the vector using the debug macro
}
```

Note that first result is the compiled binary itself

```console
    // OUTPUT from 'cargo run'
    [src/main.rs:9:5] args = [
    "target/debug/minigrep",
    ]

    $ cargo run -- needle haystack
    args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
    ]
```

#### SIDE NOTE - The args Function and Invalid Unicode

- std::env::args will panic if any argument contains invalid Unicode
- std::env::args_os instead to accept arguments containing invalid Unicode
  - returns an iterator that produces OsString values instead of String values
    - OsString values differ per platform and are more complex to work with than String values.

### Saving the Argument Values in Variables

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // COMMAND: cargo run -- test sample.txt
    // OUTPUT:
    // Searching for test
    // In file sample.txt
}
```
