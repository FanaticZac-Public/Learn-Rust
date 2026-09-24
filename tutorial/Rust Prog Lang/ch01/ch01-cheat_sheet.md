<style>
@import url("../notes.css");
</style>

# Chapter 1 - Getting Started

## rustup

```console
// update rust
$ rustup update

// uninstall rust
$ rustup self uninstall
```

## work offline

read local document

```console
rustup doc
```

install dependencies to work offline

```console
cargo new get-dependencies
cd get-dependencies
cargo add rand@0.8.5 trpl@0.2.0
```

- Can delete get-dependencies folder after.
- Use the --offline flag with all cargo commands to use these cached versions

## rust/cargo usages

Running from console, create file main.rs and set main function

```rust
fn main() {
    println!("Hello, world!");
}
```

In terminal, compile and run (linux)

```console
$ rustc main.rs
$ ./main
```

You can run 'rustfmt' to format to default standard.

```console
$ rustfmt main.rs
```

- Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

Cargo is Rust’s build system and package manager.

```console
// get version
$ cargo --version

// new project
$ cargo new hello_cargo
$ cd hello_cargo

// git files will not be created if already in a repository but can override with:
$ cargo new --vcs=git

// get all options
$ cargo new --help

// Initialize project in existing folder (if cargo new wasn't used)
cargo init

// Build project - if dependencies (from crate.io) are added to toml - this will install them also
$ cargo build

// Check for errors
$ cargo check

// Build and run project
$ cargo run

// Build release version
$ cargo build --release
```


<div class="zac-note">
Some added for reference from later chapters
</div>

```console
// Will build documentation for your project using a your toml and open it.
$ cargo doc --open
```