# Control Flow

The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

```rust
fn main() {

    //-------------- if Expressions --------------
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Unlike some languages - Rust will not try and convert
    if number {
        println!("number was three");
        // error[E0308]: mismatched types
    }

    // this works though 
    if number != 0 {
        println!("number was something other than zero");
    }


    //-------------- Handling Multiple Conditions with else if --------------
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    //-------------- Using if in a let Statement --------------
    // can do this
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Can't do this - let must have 1 resultant type
    let number = if condition { 5 } else { "six" };
    // error[E0308]: `if` and `else` have incompatible types


    //-------------- Repetition with Loops--------------

    // basic loop until ctl+C
        loop {
        println!("again!");
    }

    // Returning Values from Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // prints 20


    // Disambiguating with Loop Labels
    // you can call specific inner outer loop with label
    // - rather than break/continue referring to most inner
    let mut count = 0;
    'counting_up: loop { // specifying loop label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // referring to loop label
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    //-------------- While loop - streamlining --------------
    // Streamlining Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //-------------- For Loop --------------
    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while loop
    // Rust doesn't like numeric indexing - avoid at all costs
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for loops avoid index out of bounds runtime errors
    // - and should ALWAYS be used if able
    for element in a {
        println!("the value is: {element}");
    }

    // for look with range diction
    // range diction includes indexing technically so be careful to ensure ranges exist or it could panic (exit)
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
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
   <strong>I kept the code above as un-annotated as possible
</strong> <br>See below for my quick notes from reading chapter for more detail - or read the book yourself sheesh...
    </td>
  </tr>
</table>


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