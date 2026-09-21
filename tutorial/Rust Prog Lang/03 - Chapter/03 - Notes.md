# Chapter 03 - My First Read Notes

This section is about variables, basic types, functions, comments, and control flow. These notes are just stuff that stands out from me from the book - that i may want to refer to later in short form.

# 3.1 - Variables and Mutability

## Variables
- By default, variables are immutable
    - For safety and concurrency
    - Can opt out but putting "let mut x ..."

## Constants
- Constants are mutable too, obviously
    - but you cannot use mut
    - you use 'const' instead of 'let'
    - the type must be annotated
    - const can be declared in any scope, including global
    - must be set to a value and the result of a function
        - not the result of an expression that could only be computed at runtime (can be expression of other constants)
    - naming convention is all caps with underscores for spaces
        - const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    - are valid for full entire program execution time, within the same scope as declared
        - makes useful for globally relevant data 

## Shadowing
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```
```console
The value of x in the inner scope is: 12
The value of x is: 6
```

- Shadowing occurs where the same immutable variable name is re-defined after the first 'overshadowing' it.
    - The first is 'shadowed' by the second (terminology)
    - Note: that 'let' is used in both redefinition
    - We are not changing the value - we are creating a new one with the same name
    - By using let we can perform new transformation on a value but have the variable remain immutable after
    - When the inner scope ends the shadowed value returns
    - We can also change the type of the value with shadowing
        - If we have a variable input as text - we could just shadow that count as an int without name changes like char_count, can just set new value with type - thus not requiring multiple variable names for each type
        - If we were to use to 'mut' for this we would get wrong type error
    - 
    
# 3.2 - Data Types
- Every value in rust is a certain data type
    - 2 data type subsets are Scalar and Compound
    - Rust is statically typed language - meaning we must know all types at compile time.
    - The compiler will assume based on value provided and how we use it
        - however if many types are possible, such as with parse, 
    - See the u32 added. (error type annotations needed otherwise)
then we must set it manually
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
## Scalar Types

- Scalar repesents single value
    - integers, floating-point, numbers, booleans, characters

### Integer Types

- Integer is a number without a faractional component.

| Length              | Signed | Unsigned |
|---------------------|--------|----------|
| 8-bit               | `i8`   | `u8`     |
| 16-bit              | `i16`  | `u16`    |
| 32-bit              | `i32`  | `u32`    |
| 64-bit              | `i64`  | `u64`    |
| 128-bit             | `i128` | `u128`   |
| Architecture-dependent | `isize` | `usize` |

- Signed numbers are stored using two's complement representation
- Signed can store -2^(n - 1) to 2^(n - 1) - 1 inclusive
    - i8 can store numbers from −(2^7) to 2^7 − 1, which equals −128 to 127
- Unsigned variants can store numbers from 0 to 2n − 1
    - u8 can store numbers from 0 to 2^8 − 1, which equals 0 to 255.
- types depend on the architecture of the computer your program is running on
    - 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

#### literals syntax    

| Number literal | Example        |
|----------------|----------------|
| Decimal        | `98_222`       |
| Hex            | `0xff`         |
| Octal          | `0o77`         |
| Binary         | `0b1111_0000`  |
| Byte (u8 only) | `b'A'`         |

- integer literals can be written like above
    - number literals that can be multiple numeric types allow a type suffix to designate the type
        - such as 57u8
- Number literals can also use _ as a visual separator for readability
    - 1_000, equals 1000
- Default integer is i32
- The primary situation you’d use isize or usize is when indexing some sort of collection.

#### Integer Overflow

- In debug mode - Rust will check for integer overflow that could cause panicking at runtime. 
- Rust uses 'panicking' when program exits at runtime if this behavior occurs
    - more in chp9
- When compiling in --release mode, Rust does not include checks, instead, if overflow occurs, Rust performes "two's complement wrapping". 
    - aka Values greater than max will wrap around to the minimum it can hold. i.e. (256 becomes 1 for u8)
    - won't panic but unexpected value
    -Relying on overflow is considered an error
- To handle possibility of overflow you can use these methods provided by the standard library for primative numberic types
    - Wrap all modes with wrapping_* methods such as wrapping_add
    - Return the None value if there is overflow with the checked_* methods
    - Return the value and a Boolean indicating overflow with the overflowing_* methods
    - Saturate at the value's minimum or maximum values with the saturating_* methods
For looking at these methods, check out api [u8](https://doc.rust-lang.org/std/primitive.u8.html)

### Floating Point Types
- 2 primative types for floating-point numbers f32 and f64
    - default is f64 because on modern cpus is roughly same speed, but more precise. 
    - all floating points are signed
- Floating-point are represented to the IEEE-754 standard.

### Numeric Operations
- Rus support all basic mathematical operations
    -  addition, subtraction, multiplication, division, and remainder.
    - Integer division truncates toward zero to the nearest integer. 
        - -5 / 3 results in -1

### The Boolean Type
- Boolean type in Rust has two possible values: true and false. 
- Booleans are one byte in size. 
- The Boolean type in Rust is specified using bool

### The Character Type
- Rust’s char type is the language’s most primitive alphabetic type
```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
- specify char literals with single quotation marks
- Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.
- Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

## Compount Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple Type
```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
- A tuple groups together a number of values with a variety of types into one compound type. 
- Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
- The variable tup binds to the entire tuple because a tuple is considered a single compound element.

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```
- User pattern matching to destructure tuple values
    - This program first creates a tuple and binds it to the variable tup
    - It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
        - This is called destructuring because it breaks the single tuple into three parts. 

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
- We can also access a tuple element directly by using a period (.)
- The tuple without any values has a special name, unit.
    - This value and its corresponding type are both written () and represent an empty value or an empty return type. 
    - Expressions implicitly return the unit value if they don’t return any other value.

## The Array Type
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
- Unlike a tuple, every element of an array must have the same type.
- Unlike arrays in some other languages, arrays in Rust have a fixed length.
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```
- Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap (more in chp4)
- or when you want to ensure that you always have a fixed number of elements. 
- However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:


- An array isn’t as flexible as the vector type
- A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap.
- If you’re unsure whether to use an array or a vector, chances are you should use a vector. ch8

```rust
let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];
```
You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
```rust
let a = [3; 5];
let a = [3, 3, 3, 3, 3]; // Samezies!
```

### Array Element Access
Normal indexing
```rust
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
```
### Invalid Array Element Access
```rust
let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
```
- It's saying don't use out of bounds indexing?
- This check has to happen at runtime - so programmer awares
    - Rust cannot prevent this.
- This will cause panic situation in rust though which many low-level languages will not - and could cause memory leaks.
    - This is an example of Rust’s memory safety principles in action. 


# 3.3 - Functions
- Normal syntax, fn keyword, main() usual entry point
```rust
fn main() {
    println!("Hello, world!");

    another_function();
}
```
- Nothing unique to mention here

## Parameters
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

## Statements and Expressions
Rust is an expression-based language
- Statements are instructions that perform some action and do not return a value
- Expressions evaluate to a resultant value.
Other languages don’t have the same distinctions

### Statements
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

### Expressoins
- Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
- Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
- xpressions can be part of statements: In Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. 
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

## Functions with Return Values
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

# 3.4 - Comments

- Normal line comment syntax // 
- Document comments are discussed in chapter 14



# 3.5 - Control Flow

## If expressions
The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

- Standard if/else statements
- Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions that we discussed in the “Comparing the Guess to the Secret Number” section

- Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. 
- You must be explicit and always provide if with a Boolean as its condition.

- 'else if' statements are standard
- Only evaluates to first true (standard)
- note 'match' is effective a switch in rust

Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable:
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // prints 5
}
```

- Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. 
- This means the values that have the potential to be results from each arm of the if must be the same type
- If the types are mismatched - error[E0308]: `if` and `else` have incompatible types
- Knowing the type of number lets the compiler verify the type is valid everywhere we use number
    - runtime safety -  the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

## Repetition with Loops

### Repeating Code with loop
Rust has three kinds of loops: loop, while, and for. Let’s try each one.

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

- can break out with 'break' (standard)
- can continue looping with 'continue' (standard)

### Returning Values from Loops

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // returns 20
}
```

- Worth noting the lack of return keyword being used here
    - return is a keyword and returns from the function but it's implicit if the function runs out.
- the break keyword with the value counter * 2. After the loop, we use a semicolon to end the statement that assigns the value to result

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    //    count = 0
    //    remaining = 10
    //    remaining = 9
    //    count = 1
    //    remaining = 10
    //    remaining = 9
    //    count = 2
    //    remaining = 10
    //    End count = 2
}
```
- break and continue apply to the innermost loop
- ou can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    - So you can continue or break higher up nested

### Streamlining Conditional Loops with while

```rust 
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
-- Reduces nesting and checks - standard

### Looping Through a Collection with for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
- All five array values appear in the terminal, as expected.
- However, this approach is error-prone; we could cause the program to panic if the index value or test condition is incorrect. For example, if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.

Instead do
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```
- this eliminates issue with potential out of bounds from human error
- Summary - Rust doesn't like numeric indexing - avoid at all costs
- it's also more effecient because no comparators are required
- this is why for loops are prefered -  safety and conciseness 
- Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop. 
    - TIP The way to do that would be to use a Range, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.

This is example of range
```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

# Recommended practice
- Convert temperatures between Fahrenheit and Celsius.
- Generate the nth Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
