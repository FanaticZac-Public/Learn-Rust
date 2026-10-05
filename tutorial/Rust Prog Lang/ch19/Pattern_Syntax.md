# Pattern Syntax

## Super Summary

```rust
//-------------- Matching Literals --------------
// This syntax is useful when you want your code to take an action if it gets a particular concrete value.
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
} // one


//-------------- Matching Named Variables --------------
// Named variables are irrefutable patterns that match any value.
// - However, each of these kinds of expressions starts a new scope, variables declared as part of a pattern inside these expressions will shadow those with the same name outside the constructs.
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    // y is new variable here - overshadowing y and equal to x
    Some(y) => println!("Matched, y = {y}"), 
    _ => println!("Default case, x = {x:?}"),
}

println!("at the end: x = {x:?}, y = {y}");
// Matched, y = 5
// at the end: x = Some(5), y = 10

// - Point is to be careful when introducing variables to the match statement - as new variable names can be added in parameters for use in that specific arm - but can confuse you with overshadowing same name variables from outer scope


//-------------- Matching Multiple Patterns --------------
// In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator. 
// - '|' aka pipe equals 'or' 
let x = 1;

match x {
    1 | 2 => println!("one or two"), // 1 or 2
    3 => println!("three"),
    _ => println!("anything"),
}


//-------------- Matching Ranges of Values with ..= --------------
// The ..= syntax allows us to match to an inclusive range of values. It's similar to using '|' for 'or' but easier for long ranges.
// - ranges are only allowed with numeric or char values.
// ------ numeric -------
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
// If x is 1, 2, 3, 4, or 5, the first arm will match.
// The compiler checks that the range isn’t empty at compile time

// ------ chars -------
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
// Rust can tell that 'c' is within the first pattern’s range and prints early ASCII letter.


//-------------- Destructuring to Break Apart Values --------------
// We can also use patterns to destructure structs, enums, and tuples to use different parts of these values.

// ------ structs -------
// a Point struct with two fields, x and y, that we can break apart using a pattern with a let statement.
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // variables don't have to match struct
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // More common to match for consistency/readability
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // or use match expression for 3 cases
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }    // On the y axis at 7
}

// ------ Enums -------
// the pattern to destructure an enum corresponds to the way the data stored within the enum is defined

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Note the various destructuring.
fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}


// ------ Nested Structs and Enums -------

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
    // Change color to hue 0, saturation 160, value 255
}

// ------ Structs and Tuples -------
// We can mix, match, and nest destructuring patterns in even more complex ways.
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });


//-------------- Ignoring Values in a Pattern --------------

// ------ An Entire Value with _ -------
// - This code will completely ignore the value 3 passed as the first argument, and will print This code only uses the y parameter: 4.
// - You may not want to change the signature (like with a implementing a trait), but don't need all variables in this case
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}

// ------ Parts of a Value with a Nested _ -------
// We can also use _ inside another pattern to ignore just part of a value, for example, when we want to test for only part of a value but have no use for the other parts in the corresponding code we want to run. 
// - The business requirements are that the user should not be allowed to overwrite an existing customization of a setting but can unset the setting and give it a value if it is currently unset.
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    // we are just checking that it's both Some things, not None anything
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {setting_value:?}");

// ------ -------
// We can also use underscores in multiple places within one pattern to ignore particular values. Listing 19-19 shows an example of ignoring the second and fourth values in a tuple of five items.
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}");
    }
}
// This code will print Some numbers: 2, 8, 32, and the values 4 and 16 will be ignored. 


// ------ An Unused Variable by Starting Its Name with _ -------
// you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore. 
fn main() {
    let _x = 5;
    let y = 10;
}

// Note that there is a subtle difference between using only _ and using a name that starts with an underscore. We'll compare

// Case 1 - We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again. 
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{s:?}");

// Case 2 - using the underscore by itself doesn’t ever bind to the value, will compile without any errors because s doesn’t get moved into _.
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{s:?}");


// ------ Remaining Parts of a Value with .. -------
// we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value. 
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {x}"),
}

// Example with tuple 
// The syntax .. will expand to as many values as it needs to be. 
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    // Some numbers: 2, 32
}

// Example with error due to ambiguity
// If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error.
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
    // error: `..` can only be used once per tuple pattern
}


//-------------- Adding Conditionals with Match Guards--------------
// A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen.
// - that they are only available in match expressions, not if let or while let expressions.
let num = Some(4);

// match where the first arm has the pattern Some(x) and also has a match guard of if x % 2 == 0
match num {
    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    Some(x) => println!("The number {x} is odd"),
    None => (),
} // The number 4 is even

// The downside of this additional expressiveness is that the compiler doesn’t try to check for exhaustiveness when match guard expressions are involved.

// ------ Match guards to solve our pattern-shadowing problem (above) -------
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    // Default case, x = Some(5)

    println!("at the end: x = {x:?}, y = {y}");
    //at the end: x = Some(5), y = 10
}

// ------ You can also use the or operator | in a match guard to specify multiple patterns -------
// The important part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies to 6.
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
} // eq. (4 | 5 | 6) if y => ...


//-------------- Using @ Bindings --------------
// @ lets us create a variable that holds a value at the same time we’re testing that value for a pattern match.
// We want to test that a Message::Hello id field is within the range 3..=7. We also want to bind the value to the variable id so that we can use it in the code associated with the arm.
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id @ 3..=7 } => {
        println!("Found an id in range: {id}")
    }
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {id}"),
}
``` 

### My additional notes
- In this chapter there are 2 times illustrated when the compiler could potentially fail to check for exhaustiveness:
    - 'if let' expressions (will simply ignore all other occurrences)
    - When using Match Guards (in second tier the nested section)

## Pattern Syntax

In this section, we gather all the syntax that is valid in patterns and discuss why and when you might want to use each one.

### Matching Literals

As you saw in Chapter 6, you can match patterns against literals directly. The following code gives some examples:

```rust
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

- This code prints one because the value in x is 1. This syntax is useful when you want your code to take an action if it gets a particular concrete value.

### Matching Named Variables

- Named variables are irrefutable patterns that match any value, and we’ve used them many times in this book.
- However, there is a complication when you use named variables in match, if let, or while let expressions.
- Because each of these kinds of expressions starts a new scope, variables declared as part of a pattern inside these expressions will shadow those with the same name outside the constructs, as is the case with all variables.
- In Listing 19-11, we declare a variable named x with the value Some(5) and a variable y with the value 10. 
- We then create a match expression on the value x.
- Look at the patterns in the match arms and println! at the end, and try to figure out what the code will print before running this code or reading further.

```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
    // Matched, y = 5
    // at the end: x = Some(5), y = 10
```

- Let’s walk through what happens when the match expression runs. The pattern in the first match arm doesn’t match the defined value of x, so the code continues.

- The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. 
- Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10. 
- This new y binding will match any value inside a Some, which is what we have in x. 
- Therefore, this new y binds to the inner value of the Some in x. - That value is 5, so the expression for that arm executes and prints Matched, y = 5.

- If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore. 
- We didn’t introduce the x variable in the pattern of the underscore arm, so the x in the expression is still the outer x that hasn’t been shadowed. 
- In this hypothetical case, the match would print Default case, x = None.

- When the match expression is done, its scope ends, and so does the scope of the inner y. 
- The last println! produces at the end: x = Some(5), y = 10.

- To create a match expression that compares the values of the outer x and y, rather than introducing a new variable that shadows the existing y variable, we would need to use a match guard conditional instead. 
- We’ll talk about match guards later in the “Adding Conditionals with Match Guards” section.

### Matching Multiple Patterns

- In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.
- For example, in the following code, we match the value of x against the match arms, the first of which has an or option, meaning if the value of x matches either of the values in that arm, that arm’s code will run:

```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

### Matching Ranges of Values with ..=

The ..= syntax allows us to match to an inclusive range of values. In the following code, when a pattern matches any of the values within the given range, that arm will execute:

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

If x is 1, 2, 3, 4, or 5, the first arm will match. 
- This syntax is more convenient for multiple match values than using the | operator to express the same idea; if we were to use |, we would have to specify 1 | 2 | 3 | 4 | 5. 
- Specifying a range is much shorter, especially if we want to match, say, any number between 1 and 1,000!

- The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

Here is an example using ranges of char values:

```rust
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
```
Rust can tell that 'c' is within the first pattern’s range and prints early ASCII letter.

### Destructuring to Break Apart Values

We can also use patterns to destructure structs, enums, and tuples to use different parts of these values. Let’s walk through each value.

#### Structs

Listing 19-12 shows a Point struct with two fields, x and y, that we can break apart using a pattern with a let statement.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

- This code creates the variables a and b that match the values of the x and y fields of the p struct. 
- This example shows that the names of the variables in the pattern don’t have to match the field names of the struct. 
- However, it’s common to match the variable names to the field names to make it easier to remember which variables came from which fields. 
- Because of this common usage, and because writing let Point { x: x, y: y } = p; contains a lot of duplication, Rust has a shorthand for patterns that match struct fields: You only need to list the name of the struct field, and the variables created from the pattern will have the same names. 
- Listing 19-13 behaves in the same way as the code in Listing 19-12, but the variables created in the let pattern are x and y instead of a and b.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

- This code creates the variables x and y that match the x and y fields of the p variable. 
- The outcome is that the variables x and y contain the values from the p struct.

- We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields. - Doing so allows us to test some of the fields for particular values while creating variables to destructure the other fields.

- In Listing 19-14, we have a match expression that separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or on neither axis.

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    // On the y axis at 7
}
```

- The first arm will match any point that lies on the x axis by specifying that the y field matches if its value matches the literal 0. 
- The pattern still creates an x variable that we can use in the code for this arm.

- Similarly, the second arm matches any point on the y axis by specifying that the x field matches if its value is 0 and creates a variable y for the value of the y field. 
- The third arm doesn’t specify any literals, so it matches any other Point and creates variables for both the x and y fields.

- In this example, the value p matches the second arm by virtue of x containing a 0, so this code will print On the y axis at 7.

- Remember that a match expression stops checking arms once it has found the first matching pattern, so even though Point { x: 0, y: 0 } is on the x axis and the y axis, this code would only print On the x axis at 0.

#### Enums

- We’ve destructured enums in this book (for example, Listing 6-5 in Chapter 6), but we haven’t yet explicitly discussed that the pattern to destructure an enum corresponds to the way the data stored within the enum is defined. 
- As an example, in Listing 19-15, we use the Message enum from Listing 6-2 and write a match with patterns that will destructure each inner value.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}
```

- This code will print Change color to red 0, green 160, and blue 255. 

- For enum variants without any data, like Message::Quit, we can’t destructure the value any further. 
- We can only match on the literal Message::Quit value, and no variables are in that pattern.

- For struct-like enum variants, such as Message::Move, we can use a pattern similar to the pattern we specify to match structs. 
- After the variant name, we place curly brackets and then list the fields with variables so that we break apart the pieces to use in the code for this arm. 
- Here we use the shorthand form as we did in Listing 19-13.

- For tuple-like enum variants, like Message::Write that holds a tuple with one element and Message::ChangeColor that holds a tuple with three elements, the pattern is similar to the pattern we specify to match tuples. 
- The number of variables in the pattern must match the number of elements in the variant we’re matching.

#### Nested Structs and Enums

- So far, our examples have all been matching structs or enums one level deep, but matching can work on nested items too! 
- For example, we can refactor the code in Listing 19-15 to support RGB and HSV colors in the ChangeColor message, as shown in Listing 19-16.

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
```

- The pattern of the first arm in the match expression matches a Message::ChangeColor enum variant that contains a Color::Rgb variant; then, the pattern binds to the three inner i32 values. 
- The pattern of the second arm also matches a Message::ChangeColor enum variant, but the inner enum matches Color::Hsv instead. 
- We can specify these complex conditions in one match expression, even though two enums are involved.

#### Structs and Tuples

- We can mix, match, and nest destructuring patterns in even more complex ways. 
- The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

- This code lets us break complex types into their component parts so that we can use the values we’re interested in separately.

- Destructuring with patterns is a convenient way to use pieces of values, such as the value from each field in a struct, separately from each other.

### Ignoring Values in a Pattern

- You’ve seen that it’s sometimes useful to ignore values in a pattern, such as in the last arm of a match, to get a catch-all that doesn’t actually do anything but does account for all remaining possible values. 
- There are a few ways to ignore entire values or parts of values in a pattern: using the _ pattern (which you’ve seen), using the _ pattern within another pattern, using a name that starts with an underscore, or using .. to ignore remaining parts of a value. - Let’s explore how and why to use each of these patterns.

#### An Entire Value with _

- We’ve used the underscore as a wildcard pattern that will match any value but not bind to the value. 
- This is especially useful as the last arm in a match expression, but we can also use it in any pattern, including function parameters, as shown in Listing 19-17.

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
} 
```

- This code will completely ignore the value 3 passed as the first argument, and will print This code only uses the y parameter: 4.

- In most cases when you no longer need a particular function parameter, you would change the signature so that it doesn’t include the unused parameter. 
- Ignoring a function parameter can be especially useful in cases when, for example, you’re implementing a trait when you need a certain type signature but the function body in your implementation doesn’t need one of the parameters. 
- You then avoid getting a compiler warning about unused function parameters, as you would if you used a name instead.

#### Parts of a Value with a Nested _

- We can also use _ inside another pattern to ignore just part of a value, for example, when we want to test for only part of a value but have no use for the other parts in the corresponding code we want to run. 
- Listing 19-18 shows code responsible for managing a setting’s value. 
- The business requirements are that the user should not be allowed to overwrite an existing customization of a setting but can unset the setting and give it a value if it is currently unset.

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {setting_value:?}");
```

- This code will print Can't overwrite an existing customized value and then setting is Some(5). 
- In the first match arm, we don’t need to match on or use the values inside either Some variant, but we do need to test for the case when setting_value and new_setting_value are the Some variant. 
- In that case, we print the reason for not changing setting_value, and it doesn’t get changed.

- In all other cases (if either setting_value or new_setting_value is None) expressed by the _ pattern in the second arm, we want to allow new_setting_value to become setting_value.

- We can also use underscores in multiple places within one pattern to ignore particular values. 
- Listing 19-19 shows an example of ignoring the second and fourth values in a tuple of five items.

```rust
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
```

This code will print Some numbers: 2, 8, 32, and the values 4 and 16 will be ignored.

#### An Unused Variable by Starting Its Name with _

- If you create a variable but don’t use it anywhere, Rust will usually issue a warning because an unused variable could be a bug. - However, sometimes it’s useful to be able to create a variable you won’t use yet, such as when you’re prototyping or just starting a project. 
- In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore.
- In Listing 19-20, we create two unused variables, but when we compile this code, we should only get a warning about one of them.

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

- We’ll receive an error because the s value will still be moved into _s, which prevents us from using s again. 
- However, using the underscore by itself doesn’t ever bind to the value. Listing 19-22 will compile without any errors because s doesn’t get moved into _.

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{s:?}");
```
This code works just fine because we never bind s to anything; it isn’t moved.

#### Remaining Parts of a Value with ..

- With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value. 
- The .. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern. 
- In Listing 19-23, we have a Point struct that holds a coordinate in three-dimensional space. 
- In the match expression, we want to operate only on the x coordinate and ignore the values in the y and z fields.

```rust
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
```

- We list the x value and then just include the .. pattern. 
- This is quicker than having to list y: _ and z: _, particularly when we’re working with structs that have lots of fields in situations where only one or two fields are relevant.

- The syntax .. will expand to as many values as it needs to be. Listing 19-24 shows how to use .. with a tuple.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

- In this code, the first and last values are matched with first and last. The .. will match and ignore everything in the middle.

- However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error. Listing 19-25 shows an example of using .. ambiguously, so it will not compile.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}
// error: `..` can only be used once per tuple pattern
```
- It’s impossible for Rust to determine how many values in the tuple to ignore before matching a value with second and then how many further values to ignore thereafter. 
- This code could mean that we want to ignore 2, bind second to 4, and then ignore 8, 16, and 32; or that we want to ignore 2 and 4, bind second to 8, and then ignore 16 and 32; and so forth. 
- The variable name second doesn’t mean anything special to Rust, so we get a compiler error because using .. in two places like this is ambiguous.

### Adding Conditionals with Match Guards

- A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen. 
- Match guards are useful for expressing more complex ideas than a pattern alone allows. 
- Note, however, that they are only available in match expressions, not if let or while let expressions.

- The condition can use variables created in the pattern. 
- Listing 19-26 shows a match where the first arm has the pattern Some(x) and also has a match guard of if x % 2 == 0 (which will be true if the number is even).

```rust
let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
```

- This example will print The number 4 is even. 
- When num is compared to the pattern in the first arm, it matches because Some(4) matches Some(x). 
- Then, the match guard checks whether the remainder of dividing x by 2 is equal to 0, and because it is, the first arm is selected.

- If num had been Some(5) instead, the match guard in the first arm would have been false because the remainder of 5 divided by 2 is 1, which is not equal to 0. 
- Rust would then go to the second arm, which would match because the second arm doesn’t have a match guard and therefore matches any Some variant.

- There is no way to express the if x % 2 == 0 condition within a pattern, so the match guard gives us the ability to express this logic. 
- The downside of this additional expressiveness is that the compiler doesn’t try to check for exhaustiveness when match guard expressions are involved.

- When discussing Listing 19-11, we mentioned that we could use match guards to solve our pattern-shadowing problem. 
- Recall that we created a new variable inside the pattern in the match expression instead of using the variable outside the match. 
- That new variable meant we couldn’t test against the value of the outer variable. Listing 19-27 shows how we can use a match guard to fix this problem.

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}
// Default case, x = Some(5)
// at the end: x = Some(5), y = 10
```

- This code will now print Default case, x = Some(5). 
- The pattern in the second match arm doesn’t introduce a new variable y that would shadow the outer y, meaning we can use the outer y in the match guard. 
- Instead of specifying the pattern as Some(y), which would have shadowed the outer y, we specify Some(n). 
- This creates a new variable n that doesn’t shadow anything because there is no n variable outside the match.

- The match guard if n == y is not a pattern and therefore doesn’t introduce new variables. 
- This y is the outer y rather than a new y shadowing it, and we can look for a value that has the same value as the outer y by comparing n to y.

- You can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns. 
- Listing 19-28 shows the precedence when combining a pattern that uses | with a match guard. 
- The important part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies to 6.

```rust
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
```

- The match condition states that the arm only matches if the value of x is equal to 4, 5, or 6 and if y is true. 
- When this code runs, the pattern of the first arm matches because x is 4, but the match guard if y is false, so the first arm is not chosen. 
- The code moves on to the second arm, which does match, and this program prints no. 
- The reason is that the if condition applies to the whole pattern 4 | 5 | 6, not just to the last value 6. 
- In other words, the precedence of a match guard in relation to a pattern behaves like this:

``` (4 | 5 | 6) if y => ...```

rather than this:

```4 | 5 | (6 if y) => ...```

- After running the code, the precedence behavior is evident: If the match guard were applied only to the final value in the list of values specified using the | operator, the arm would have matched, and the program would have printed yes.

### Using @ Bindings

The at operator @ lets us create a variable that holds a value at the same time we’re testing that value for a pattern match. In Listing 19-29, we want to test that a Message::Hello id field is within the range 3..=7. We also want to bind the value to the variable id so that we can use it in the code associated with the arm.

```rust
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
```

- This example will print Found an id in range: 5. 
- By specifying id @ before the range 3..=7, we’re capturing whatever value matched the range in a variable named id while also testing that the value matched the range pattern.

- In the second arm, where we only have a range specified in the pattern, the code associated with the arm doesn’t have a variable that contains the actual value of the id field. 
- The id field’s value could have been 10, 11, or 12, but the code that goes with that pattern doesn’t know which it is. 
- The pattern code isn’t able to use the value from the id field because we haven’t saved the id value in a variable.

- In the last arm, where we’ve specified a variable without a range, we do have the value available to use in the arm’s code in a variable named id. 
- The reason is that we’ve used the struct field shorthand syntax. 
- But we haven’t applied any test to the value in the id field in this arm, as we did with the first two arms: Any value would match this pattern.

- Using @ lets us test a value and save it in a variable within one pattern.