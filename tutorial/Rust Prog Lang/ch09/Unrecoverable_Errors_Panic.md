# Unrecoverable Errors with panic!

There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro

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
   <strong>... or when giving a public presentation.
</strong>
    </td>
  </tr>
</table>

## Super Summarize
```rust
//-------------- Explicitly calling panic macro --------------
fn main() {
    panic!("crash and burn");
}
// thread 'main' panicked at src/main.rs:2:5:
// crash and burn

//-------------- Inexplicably causing a panic! --------------
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
// thread 'main' panicked at src/main.rs:4:6:
// index out of bounds: the len is 3 but the index is 99

```

## Unrecoverable Errors with panic!

There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro.

By default, these panics will print a failure message, unwind, clean up the stack, and quit.

Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.

### Unwinding the Stack or Aborting in Response to a Panic

By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, walking back and cleaning up is a lot of work. Rust therefore allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.

For example, if you want to abort on panic in release mode, add this:

```toml
[profile.release]
panic = 'abort'
```

Let’s try calling panic! in a simple program:

Filename: src/main.rs

```rust
fn main() {
    panic!("crash and burn");
    // thread 'main' (1311930) panicked at src/main.rs:2:5:
    // crash and burn
}
```

In other cases, the panic! call might be in code that our code calls, and the filename and line number reported by the error message will be someone else’s code where the panic! macro is called, not the line of our code that eventually led to the panic! call.

We can use the backtrace of the functions the panic! call came from to figure out the part of our code that is causing the problem.

To understand how to use a panic! backtrace, let’s look at another example and see what it’s like when a panic! call comes from a library because of a bug in our code instead of from our code calling the macro directly.

has some code that attempts to access an index in a vector beyond the range of valid indexes.

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99];
    // thread 'main' (1313692) panicked at src/main.rs:7:6:
    // index out of bounds: the len is 3 but the index is 99
}
```

- Out of index example - i put the code from my execution

In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a 'buffer overread' and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.

The note: line tells us that we can set the RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error. A backtrace is a list of all the functions that have been called to get to this point.

The key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above that spot are code that your code has called; the lines below are code that called your code. These before-and-after lines might include core Rust code, standard library code, or crates that you’re using. Let’s try to get a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0.

Example:

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

ine 6 of the backtrace points to the line in our project that’s causing the problem: line 4 of src/main.rs. If we don’t want our program to panic, we should start our investigation at the location pointed to by the first line mentioning a file we wrote.

where we deliberately wrote code that would panic, the way to fix the panic is to not request an element beyond the range of the vector indexes. When your code panics in the future, you’ll need to figure out what action the code is taking with what values to cause the panic and what the code should do instead.
