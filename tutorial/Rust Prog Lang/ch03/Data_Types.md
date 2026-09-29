# Data Types

Rust is *statically typed* language - meaning we must know all types at compile time.

The compiler will infer type based on value or how we use it
- however if many types are possible, such as with parse, then we must set it manually
- error[E0284]: type annotations needed

```rust
    // Explicit type declaration because .parse() can return several, and compiler needs to know
    let guess: u32 = "42".parse().expect("Not a number!");


    // ////////////// SCALAR //////////////
    
    //-------------- Floating Point --------------
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32


    //-------------- Numeric Operations --------------
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    //-------------- The Boolean Type --------------
    let t = true;

    let f: bool = false; // with explicit type annotation


    //-------------- The Boolean Type --------------
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // ////////////// COMPOUND //////////////

    //-------------- The Tuple Type --------------
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // OR 
    let tup = (500, 6.4, 1);

    // The tuple without any values has a special name, unit.
    let tup: () = ();
   
    // pattern matching to destructure a tuple value
    let (x, y, z) = tup;

    // using a period (.)
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //-------------- The Array Type --------------
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];             // Samezies!
    let a = [3, 3, 3, 3, 3];    // Samezies!

    // Array Element Access
    let first = a[0];
    let second = a[1];

    // Invalid Array Element Access
    let element = a[index]; // if index is out of bounds

    // NOTE:
    // Unique feature of Rust: it will check at runtime that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic (exit). (Other low level languages could result in undefined behavior and security flaws)
    // It's recommended later to use iterator functions with .get to avoid this entirely, aka index out of bounds errors. aka ch09. 

```

There are 2 data type subsets are Scalar and Compound.

## Scalar repesents single value
- integers, floating-point, numbers, booleans, characters

### Integer Types

| Length              | Signed | Unsigned |
|---------------------|--------|----------|
| 8-bit               | `i8`   | `u8`     |
| 16-bit              | `i16`  | `u16`    |
| 32-bit              | `i32`  | `u32`    |
| 64-bit              | `i64`  | `u64`    |
| 128-bit             | `i128` | `u128`   |
| Architecture-dependent | `isize` | `usize` |

- Integer is a number without a fractional component.
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
- 2 primitive types for floating-point numbers f32 and f64
    - default is f64 because on modern CPUs is roughly same speed, but more precise. 
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
- specify char literals with single quotation marks
- Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.
- Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.

## Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple Type

- A tuple groups together a number of values with a variety of types into one compound type. 
- Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
- The variable tup binds to the entire tuple because a tuple is considered a single compound element.

- Use pattern matching to destructure tuple values
    - This program first creates a tuple and binds it to the variable tup
    - It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
        - This is called destructuring because it breaks the single tuple into three parts. 

- We can also access a tuple element directly by using a period (.)
- The tuple without any values has a special name, unit.
    - This value and its corresponding type are both written () and represent an empty value or an empty return type. 
    - Expressions implicitly return the unit value if they don’t return any other value.

### The Array Type
An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
- Unlike a tuple, every element of an array must have the same type.
- Unlike arrays in some other languages, arrays in Rust have a fixed length.

- Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap (more in chp4)
- or when you want to ensure that you always have a fixed number of elements. 
- However, arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:


- An array isn’t as flexible as the vector type
- A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap.
- If you’re unsure whether to use an array or a vector, chances are you should use a vector. ch8

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
- This check has to happen at runtime - so programmer aware
    - Rust cannot prevent this at compile time.
- This will cause panic situation in rust though which many low-level languages will not - and could cause memory leaks.
    - This is an example of Rust’s memory safety principles in action. 
