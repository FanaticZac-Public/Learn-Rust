# Functions


```rust
// ////////////// FUNCTIONS //////////////

//-------------- Basic Syntax --------------
// Normal syntax, fn keyword, main() usual entry point
fn main() {
    println!("Hello, world!");

    // Calling another function
    another_function();
}

fn another_function() {
    println!("Another function.");
}


//-------------- Parameters --------------
fn main() {
    // calling with parameter
    another_function(5);
}

// In function signatures, you must declare the type of each parameter. 
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


// Function Signatures
fn main() {
    // calling with parameter
    print_labeled_measurement(5, 'h');
}

// Multiple parameters - different types
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// ////////////// STATEMENTS AND EXPRESSIONS //////////////
// --- rust specific stuff - the distinction


//-------------- Statements --------------
//  Statements are instructions that perform some action and do not return a value

fn main() {
    let y = 6; // statement (no return value)
}

//  Statements do not return values.
// - Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
fn main() {
    let x = (let y = 6);
    /// error: expected expression, found `let` statement
}
// The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. 
// Unlike other languages, you cannot you can write x = y = 6


//-------------- Expressions --------------
// Expressions evaluate to a resultant value.
// Expressions can be part of statements
// Expressions do not include ending semicolons.
// Calling a macro is an expression... (see aside at end)

// the 6 in the statement let y = 6; is an expression that evaluates to the value 6. 
let y = 6; // the 6 only

let y = (6 + 2); // (6 + 2) is an expression

// Calling a function is an expression.
calling_function()



// A new scope block created with curly brackets is an expression, for example:
```rust 
fn main() {
    let y = {
        let x = 3;
        x + 1
    }; 
    // let y = ; // is the statement
    // This is the expression:
    // {
    //     let x = 3;
    //     x + 1
    // }   

    println!("The value of y is: {y}");
    // returns 4
}


//-------------- Functions with Return Values --------------
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. - no return required

fn five() -> i32 {
    5           // Returned implicitly - no semi colon means expression returning value
}

fn main() {
    let x = five(); // returns 5 to x.

    println!("The value of x is: {x}");
}

// same thing here:
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    // returns 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
// if we were to place a semi-colon on the 'x + 1' line we would get 'error[E0308]: mismatched types' because we aren't returning anything as that makes it a statement.

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
   <strong>An aside by me
</strong><br>Because how is a macro an expression?!...
    </td>
  </tr>
</table>

```rust
// -------------- An Aside by me --------------
// Calling a macro is an expression... (what...)

println!("{0}", "Because I return something") // ! is the macro char

// FYI this confused me - because it's printing to stdout - but it's technically returning () so it's an expression.

// heres how a macro works under the hood:
macro_rules! create_function {
    () => {
        fn hello() {
            println!("Hello");
        }
    };
} // so it technically returns ().

// If you add a semicolon to the end of an expression, you turn it into a statement (with expression inside), and it will then not return a value to the outer function holding println!.
println!("{0}", "Because I return something"); // statement now
// in short ; tells the compiler nothing is being returned. 

// heres how a macro works under the hood:
macro_rules! create_function {
    () => {
        fn hello() {
            println!("Hello");
        }
    };
}// so it returns ().
// so create_function!(); is a statements with expression... 
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
   <strong>Rust employs functional programming principles
</strong><br>Which is also relevant with the statement/expression section<br>
        Functions can be called in statements - talked about in later chapters
    </td>
  </tr>
</table>



## Functions Notes
- In function signatures, you must declare the type of each parameter. 
    - helps compiler, helps bug detection
- comma separated


### Parameters
```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
- example - pretty standard stuff
- In function signatures, you must declare the type of each parameter. 
    - helps compiler, helps bug detection
- comma seperated

### Statements and Expressions
Rust is an expression-based language
- Statements are instructions that perform some action and do not return a value
- Expressions evaluate to a resultant value.
Other languages don’t have the same distinctions

#### Statements
```rust
fn main() {
    let y = 6;
}
```
- this let line is a statement
- Function definitions are also statements so the whole example is a statement as well.
- Statements do not return values.
- Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
```rust
fn main() {
    let x = (let y = 6);
}
/// error: expected expression, found `let` statement
```
- The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. 
- This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.
    - In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

#### Expressions
- Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
- Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
- expressions can be part of statements: In Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. 
- Calling a function is an expression.
- Calling a macro is an expression.
A new scope block created with curly brackets is an expression, for example:
```rust 
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // returns 4
}
```
- Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. 
    - expressions do not include ending semicolons.
    -  If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

### Functions with Return Values
- Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).
- In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
- You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
```rust 
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```
- There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. 
    - That’s a perfectly valid function in Rust.
    - let x = 5; // is the same
```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
- Running this code will print The value of x is: 6. 
- if we place a semi-colon on the 'x + 1' line we would get error:
    - error[E0308]: mismatched types
    - because we aren't returning avalue as it's a statement.