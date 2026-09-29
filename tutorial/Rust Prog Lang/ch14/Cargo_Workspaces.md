

## Cargo Workspaces

If your library crate continues to get bigger and you want to split your package further into multiple library crates then Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.

### Creating a Workspace

A workspace is a set of packages that share the same Cargo.lock and output directory.

There are multiple ways to structure a workspace, so we’ll just show one common way.

The binary, which will provide the main functionality, will depend on the two libraries.

- One library will provide an add_one function
- One library an add_two function.

These three crates will be part of the same workspace.

```console
$ mkdir add
$ cd add
```

Add the Cargo.toml file manually and add:

```toml
[workspace]
resolver = "3"
```

Create the `adder` binary crate by running `cargo new` within the add directory:

```console
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

Running cargo new inside a workspace also automatically adds the newly created package to the members key in the [workspace] definition in the workspace Cargo.toml, like this:

```toml
[workspace]
resolver = "3"
members = ["adder"]
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
It did!
</strong>
    </td>
  </tr>
</table>


File structure looks like :

```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one target directory at the top level that the compiled artifacts will be placed into; the adder package doesn’t have its own target directory.

- Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in add/target rather than add/adder/target.

By sharing one target directory, the crates can avoid unnecessary rebuilding.

### Creating the Second Package in the Workspace

```console
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

The top-level Cargo.toml will now include the add_one path in the members list:

```toml
[workspace]
resolver = "3"
members = ["adder", "add_one"]
```

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

In the add_one/src/lib.rs file, let’s add an add_one function:

```rust
// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Now we can have the adder package with our binary depend on the add_one package that has our library. First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.

Filename: adder/Cargo.toml

```toml
[dependencies]
add_one = { path = "../add_one" }
```

Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships.

Next, let’s use the add_one function (from the add_one crate) in the adder crate.

```rust
// Filename: adder/src/main.rs
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

Let’s build the workspace by running cargo build in the top-level add directory!

```console
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

To run the binary crate from the add directory, we can specify which package in the workspace we want to run by using the -p argument and the package name with cargo run:

```console

$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

This runs the code in adder/src/main.rs, which depends on the `add_one` crate.

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
Seems simple enough - so i can build as many library crates as i want locally (since there's only 1 allowed per crate) - and link their functions via local dependencies in the callers (binary crate) Cargo.toml file. 
</strong>
    </td>
  </tr>
</table>

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
I'm not exactly sure why that can't be done without the workspace Cargo.toml - but I guess the benefit is you have on target for the whole project in the workspace level folder. <br>
- I suppose one benefit is not having to build them all separately whenever I'm working across them.<br>
- Also it means you are only downloading dependencies once if they're used for multiple crates in a workspace(next section)<br>
- Also it means all tests occur at once across workspace
</strong>
    </td>
  </tr>
</table>



### Depending on an External Package

Notice that the workspace has only one Cargo.lock file at the top level, rather than having a Cargo.lock in each crate’s directory.

- This ensures that all crates are using the same version of all dependencies. If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. Making all crates in the workspace use the same dependencies means the crates will always be compatible with each other.

Let’s add the rand crate to the [dependencies] section in the add_one/Cargo.toml file so that we can use the rand crate in the add_one crate:

Filename: add_one/Cargo.toml

```toml
[dependencies]
rand = "0.8.5"
```

We can now add `use rand;` to the add_one/src/lib.rs file, and building the whole workspace by running cargo build in the add directory will bring in and compile the rand crate. We will get one warning because we aren’t referring to the rand we brought into scope:

// Cargo.lock

```toml
[[package]]
name = "add_one"
version = "0.1.0"
dependencies = [
 "rand",
]
```

The top-level Cargo.lock now contains information about the dependency of add_one on rand. However, even though rand is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well.

For example, if we add use rand; to the adder/src/main.rs file for the adder package, we’ll get an error:

```console
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

To fix this, edit the Cargo.toml file for the adder package and indicate that rand is a dependency for it as well.

**Building the adder package will add rand to the list of dependencies for adder in Cargo.lock, but no additional copies of rand will be downloaded.**

If crates in the workspace specify incompatible versions of the same dependency, Cargo will resolve each of them but will still try to resolve as few versions as possible.

### Adding a Test to a Workspace

For another enhancement, let’s add a test of the add_one::add_one function within the add_one crate:

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Now run cargo test in the top-level add directory. Running cargo test in a workspace structured like this one will run the tests for all the crates in the workspace:

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
I had to add --lib flag but uh-huh, sure did.
</strong>
    </td>
  </tr>
</table>

```console
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

We can also run tests for one particular crate in a workspace from the top-level directory by using the -p flag and specifying the name of the crate we want to test:

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
I had to add --lib flag but uh-huh, sure did.
</strong>
    </td>
  </tr>
</table>

```console
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



**If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately.** Like cargo test, we can publish a particular crate in our workspace by using the -p flag and specifying the name of the crate we want to publish.

For additional practice, add an add_two crate to this workspace in a similar way as the add_one crate!
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
Nah. I'm good i get it.</strong>
    </td>
  </tr>
</table>
 
