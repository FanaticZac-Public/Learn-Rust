use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // test 1 - forcing panic manually
    // panic!("crash and burn");

    // Output:
    // thread 'main' (1311930) panicked at src/main.rs:2:5:
    // crash and burn

    // test 2 - imitating bug - "Index out of bounds"
    // let v = vec![1, 2, 3];
    // v[99];

    // Output:
    // thread 'main' (1313692) panicked at src/main.rs:7:6:
    // index out of bounds: the len is 3 but the index is 99
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // test 3 - same as test 2 but with `RUST_BACKTRACE=1` environment variable
    // Running this below in terminal
    // $ RUST_BACKTRACE=1 cargo run

    // Output:
    // 7: <fn() as core::ops::function::FnOnce<()>>::call_once
    // at /rustc/48a229ceaefd4985c50990b14116b6d856af0985/library/core/src/ops/function.rs:250:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    // test 4 - same as test 2 but with `RUST_BACKTRACE=full` environment variable
    // Running this below in terminal
    // $ RUST_BACKTRACE=full cargo run

    // Output - mine was long - but example from text is here:
    // ALSO - start from top read down until you find your files to find point of failure - it's numbered in rust. reverse order of events
    //     stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/std/src/panicking.rs:692:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:75:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/4d91de4e48198da2e33413efdcd9cd2cc0c46688/library/core/src/panicking.rs:273:5
    //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //              at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
    //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //              at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
    //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    //              at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3361:9
    //    6: panic::main
    //              at ./src/main.rs:4:6
    //    7: core::ops::function::FnOnce::call_once
    //              at file:///home/.rustup/toolchains/1.85/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    // test 5 - handle recoverable error file access
    // unhandled
    // let greeting_file_result = File::open("hello.txt");

    // handled
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
    // Notes: first actually only gives warning however i suspect
    // compiler would complain if i was trying to do anything with it
    // second panicked as expected.

    // test 6 - showing reason for failure rather than general
    // FYI added use std::io::ErrorKind; above
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // test 7 - Using .unwrap_or_else() syntax
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // test 8 - using .unwrap() ---EZ
    // let greeting_file = File::open("hello.txt").unwrap();
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // test 9 - using .expect()
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
    // hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // test 10 -  propagation error handling
    // use std::io::{self, Read}; // added at top

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let username_file_result = File::open("hello.txt");

    //     let mut username_file = match username_file_result {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut username = String::new();

    //     match username_file.read_to_string(&mut username) {
    //         Ok(_) => Ok(username),
    //         Err(e) => Err(e),
    //     }
    // }
    // - returns first error or second error if first succeeds, back to caller

    // test 11 - propagation with ?
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username_file = File::open("hello.txt")?;
    //     let mut username = String::new();
    //     username_file.read_to_string(&mut username)?;
    //     Ok(username)
    // }
    // - propagation is so common that returning result is marked with ?

    // test 12 - propagation with ? daisy chained
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut username = String::new();

    //     File::open("hello.txt")?.read_to_string(&mut username)?;

    //     Ok(username)
    // }

    // test 13 - Easiest way - but without explaining errors
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     fs::read_to_string("hello.txt")
    // }

    // test 14 - Using ? for Option enum instead of result
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    // - basically like a null check. 

}

// test 15 - main function usage with ? 

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
