# Recoverable Errors with Result

## Super Summarize

```rust
//-------------- Recoverable Errors with Result --------------
// built in enum for handling error situations.
// Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms. (The prelude Rust pre-imported definitions)

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// the file might not exist, or we might not have permission to access the file
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

//-------------- Matching on Different Errors --------------
use std::fs::File;
use std::io::ErrorKind; // added - used below

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}


//-------------- Alternatives to Using match with Result<T, E> --------------
// - unwrap_or_else - with closure (looked at later)
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}


//-------------- Shortcuts for Panic on Error --------------
// unwrap() is quick and dirty raw output.
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
// thread 'main' panicked at src/main.rs:4:49:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }


// .expect() is simple custom message - rather than default
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
// thread 'main' panicked at src/main.rs:5:10:
// hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }


//-------------- Propagating Errors --------------
// When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do. 
// - This is known as propagating the error
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // Returning early if/with this error
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    } // or returning if/with this Result (ok, err)
}

//-------------- The ? Operator Shortcut --------------
// Same implementation of read_username_from_file but this implementation uses the ? operator.
//  If Result is an Ok, the value inside will get returned from this expression to the variable, and the program will continue. 
// If Err, the Err will be returned from the whole function as if we had used the return keyword so that the error value gets propagated to the calling code.
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // ? means return from function early if error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // ? means return early if error
    Ok(username)
}

// Example with daisy chained using ? 
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Even shorter version using fs method
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


//-------------- Where to Use the ? Operator --------------
// can only use on functions that return Result ... or Option<> FYI
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
// - can change return type of your function to resolve


// ? can be used on Option<T> too - for Some and None
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() // keeps iterating until next equals None
}

// Back to first one
// main can also return a Result<(), E> if we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile.
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // Ignore Box<dyn Error> for now
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
// When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value. 

// Executables written in C return integers when they exit: Programs that exit successfully return the integer 0, and programs that error return some integer other than 0. Rust also returns integers from executables to be compatible with this convention.
```



## Recoverable Errors with Result

Most errors aren’t serious enough to require the program to stop entirely. Most errors aren’t serious enough to require the program to stop entirely. Sometimes when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.

Most errors aren’t serious enough to require the program to stop entirely.

Result enum is defined as having two variants, Ok and Err:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

What you need to know right now is that T represents the type of the value that will be returned in a success case within the Ok variant, and E represents the type of the error that will be returned in a failure case within the Err variant. Because Result has these generic type parameters, we can use the Result type and the functions defined on it in many different situations where the success value and error value we want to return may differ.

Let’s call a function that returns a Result value because the function could fail. In Listing 9-3, we try to open a file.

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

The return type of File::open is a Result<T, E>. The generic parameter T has been filled in by the implementation of File::open with the type of the success value, std::fs::File, which is a file handle. The type of E used in the error value is std::io::Error. This return type means the call to File::open might succeed and return a file handle that we can read from or write to. The function call also might fail: For example, the file might not exist, or we might not have permission to access the file. The File::open function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the Result enum conveys.

In the case where File::open succeeds, the value in the variable greeting_file_result will be an instance of Ok that contains a file handle.

Note that, like the Option enum, the Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.

### Matching on Different Errors

The code in Listing 9-4 will panic! no matter why File::open failed.

However, we want to take different actions for different failure reasons. If File::open failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If File::open failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to panic! in the same way it did in Listing 9-4. For this, we add an inner match expression, shown in Listing 9-5.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}
```

The type of the value that File::open returns inside the Err variant is io::Error, which is a struct provided by the standard library. This struct has a method, kind, that we can call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet. So, we match on greeting_file_result, but we also have an inner match on error.kind().

- _There's more detail in the book but this is standard error handling in terms of composition - so i'm going to skip_

#### Alternatives to Using match with Result<T, E>

The match expression is very useful but also very much a primitive. In Chapter 13, you’ll learn about closures, which are used with many of the methods defined on Result<T, E>. These methods can be more concise than using match when handling Result<T, E> values in your code.

For example, here’s another way to write the same logic as shown in Listing 9-5, this time using closures and the unwrap_or_else method:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

Although this code has the same behavior as Listing 9-5, it doesn’t contain any match expressions and is cleaner to read.

#### Shortcuts for Panic on Error

The Result<T, E> type has many helper methods defined on it to do various, more specific tasks.

The unwrap method is a shortcut method implemented just like the match expression

If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();

    // thread 'main' panicked at src/main.rs:4:49:
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}
```

- _way easier_

Similarly, the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    //     thread 'main' panicked at src/main.rs:5:10:
    // hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}
```

- In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed. That way, if your assumptions are ever proven wrong, you have more information to use in debugging.

### Propagating Errors

When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

- return type of the function first: Result<String, io::Error>
- returns on first failure or on second failure if respective error occurs
- This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

#### The ? Operator Shortcut

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    // - propagation is so common that returning result is marked with ?

}
```

- ? will return ok to the line, but err will return to the function caller
- Error values that have the ? operator called on them go through the from function, defined in the 'From' trait in the standard library, which is used to convert values from one type into another.
  - When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.
  - This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
  - _in short - it's more dynamic then the original propagation example._
- For example, we could change the read_username_from_file function in Listing 9-7 to return a custom error type named OurError that we define. If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error, then the ? operator calls in the body of read_username_from_file will call from and convert the error types without needing to add any more code to the function.
  - _this is good exercise example_

The ? operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the ?

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```

- daily chained calles with ?

Even shorter now

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

- Common way - but lacks custom error explanations.

#### Where to Use the ? Operator

The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.

- The return type of the function has to be a Result so that it’s compatible with this return.
- If used on the main function you will get error - error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`) - Because there is an implicit result and we don't want to return from main.
  This error points out that we’re only allowed to use the ? operator in a function that returns Result, Option, or another type that implements FromResidual.

two choices:

1. To fix the error, you have two choices. One choice is to change the return type of your function to be compatible with the value you’re using the ? operator on as long as you have no restrictions preventing that.
2. The other choice is to use a match or one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.

The error message also mentioned that ? can be used with Option<T> values as well.

- As with using ? on Result, you can only use ? on Option in a function that returns an Option.
- The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>:
  - If the value is None, the None will be returned early from the function at that point.
  - If the value is Some, the value inside the Some is the resultant value of the expression, and the function continues.

Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.

So far, all the main functions we’ve used return (). The main function is special because it’s the entry point and exit point of an executable program, and there are restrictions on what its return type can be for the program to behave as expected.

Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10, but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile.

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed because it allows any Err value to be returned early.

**When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value.**

Executables written in C return integers when they exit: Programs that exit successfully return the integer 0, and programs that error return some integer other than 0. _Rust_ also returns integers from executables to be compatible with this convention.

The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. Consult the standard library documentation for more information on implementing the Termination trait for your own types.