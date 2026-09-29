<style>
@import url("../notes.css");
</style>

## Installing Binaries with cargo install

The `cargo install` command allows you to install and use **binary crates** locally.

This is meant to be a convenient way for Rust developers to install tools that others have shared on crates.io.

All binaries installed with `cargo install` are stored in the installation root’s bin folder. 
- If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin
    - Ensure that this directory is in your $PATH to be able to run programs you’ve installed with cargo install.

To install ripgrep, we can run the following:
```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```
<div class="zac-note">
Ok - i did it. On linux it was in 
</div>

```console
~/.cargo/bin$ ls
cargo         cargo-miri     rls            rustdoc   rust-gdbgui
cargo-clippy  clippy-driver  rust-analyzer  rustfmt   rust-lldb
cargo-fmt     rg             rustc          rust-gdb  rustup
```

<div class="zac-note">
It's the rg one. Usage below.
</div>

```console
~/.cargo/bin$ ls | rg rust
rust-analyzer
rustc
rustdoc
rustfmt
rust-gdb
rust-gdbgui
rust-lldb
rustup
```
