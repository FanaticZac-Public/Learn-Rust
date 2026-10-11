# Redirecting Errors to Standard Error

## Super Summary

```rust
// If for example we redirect our program output to a file, like so:
// $ cargo run > output.txt
// We would still want our errors to appear in console.
// We can ensure our error messages use stderr instead of stout with
// eprintln!() instead of println!()

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
// now our standard output on success goes to file.
// our standard error goes to terminal if our program does run with a non-zero status (aka we can properly detect an error as human or system)
```

## Redirecting Errors to Standard Error

2 Kinds of terminal output:
1. stdout (Standard Output)
    - println!() uses this
2. sterr (Standard Error)

```console
// Directing standard output (stout) to file
$ cargo run > output.txt

// This will not direct stderr to file.
```

### Printing Errors to Standard Error
```rust
// printing to stderr instead in Rust
     eprintln!("Problem parsing arguments: {err}");
```
```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```