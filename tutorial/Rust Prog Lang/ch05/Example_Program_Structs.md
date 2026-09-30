# An Example Program Using Structs

## Super Summary

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
   <strong>I've added Super Summary section just for the syntax from the chapter.
</strong>
    </td>
  </tr>
</table>

```rust
//-------------- Struct Usage --------------

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
// The area of the rectangle is 1500 square pixels.
// fn area(width: u32, height: u32) -> u32 {
// - multiple parameters


//-------------- Refactoring with Tuples --------------
// passing 1 argument instead
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
// - 1 parameter but fields in function aren't explained

//-------------- Refactoring with Structs --------------
// We use structs to add meaning by labeling the data.
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Full illustrative - on big application this is handy

//-------------- Functionality with Derived Traits --------------
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
}
// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
// println!("rect1 is {rect1:?}");
// - '?:' indicates we want to print debug also
// error[E0277]: `Rectangle` doesn't implement `Debug`

// If we add derived trait - we give Rectangle debug powers
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// would print:
// rect1 is Rectangle { width: 30, height: 50 }

// Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
// - Note: Calling the dbg! macro prints to the standard error console stream (stderr)

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
// [src/main.rs:14:5] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }
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
   <strong>And below is my original note summary.
</strong><br>Still more condensed then the book - but more verbose then all the code examples with annotations
    </td>
  </tr>
</table>

## An Example Program Using Structs

To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle

- We’ll start by using single variables and then refactor the program until we’re using structs instead.

### Example without structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group width and height together.

### Refactoring with Tuples

Slightly better function signature

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way, this version is less clear: Tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

- Mixing up the width and height wouldn’t matter for the area calculation, but if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. This would be even harder for someone else to figure out and keep in mind if they were to use our code. Because we haven’t conveyed the meaning of our data in our code, it’s now easier to introduce errors.

### Refactoring with Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

- yeah yeah - we have informative signature, and data attribute names.

### Adding Functionality with Derived Traits

Example showing adjusted macro for displaying struct class with print

- _Effectively showing a purpose of interface:_
- But rust has debug feature for default

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
}
```

to this

```rust

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
    // rect1 is Rectangle { width: 30, height: 50 }
}

```

Another way to print out a value using the Debug format is to use the dbg! macro, **which takes ownership of an expression (as opposed to println!, which takes a reference)**, prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

- **Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // Outputs: [src/main.rs:87:16] 30 * scale = 60

    dbg!(&rect1);
    // outputs: [src/main.rs:91:5] &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
}
```

- We can put dbg! around the expression 30 \* scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there.
- We don’t want dbg! to take ownership of rect1 (because there is no expression to change value), so we use a reference to rect1 in the next call.

Other traits are listed in [Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
