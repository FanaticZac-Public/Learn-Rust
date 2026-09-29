<style>
@import url("../notes.css");
</style>



## Customizing Builds with Release Profiles

In Rust, release profiles are predefined, customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

Cargo has two main profiles:

1. Dev profile - when you run cargo build
2. Release profile - when you run cargo build --release.

The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

```console
// These profile names might be familiar from the output of your builds:

$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

Cargo has default settings for each of the profiles that apply when you haven’t explicitly added any [profile.*] sections in the project’s Cargo.toml file.

By adding [profile.*] sections for any profile you want to customize, you override any subset of the default settings.

```rust
// Filename: Cargo.toml
// The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.
// Applying more optimizations extends compiling time

// e.g. default values for each
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

See [Profiles - The Cargo Book](https://doc.rust-lang.org/cargo/reference/profiles.html)

<div class="zac-note">
    For example - book shows:<br>
    2 other built-in profiles: <br>
    - test and bench.<br>
    And it shows 2 other optimization instructions:<br>
    - "s": optimize for binary size<br>
    - "z": optimize for binary size, but also turn off loop vectorization.<br>
</div>
