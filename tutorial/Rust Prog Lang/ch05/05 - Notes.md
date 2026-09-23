# Chapter 05 - Using Structs to Structure Related Data 

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. 

# 5.1 Defining and Instantiating Structs

- Like tuples, the pieces of a struct can be different types. 
- Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
- Adding these names means that structs are more flexible than tuples: 
    - You don’t have to rely on the order of the data to specify or access the values of an instance.
- RUST - Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

This is all pretty standard C/c++ style stuff

- Defining
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

- Accessing Fields
```rust
   user1.email = String::from("anotheremail@example.com");
```

- Returning struct from function
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

### Using the Field Init Shorthand

- Let's you reduce the repetition
```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances with Struct Update Syntax

- Shows building user from struct without function definition - and using attributes from previous user.
```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

- Let's you use short hand using the range symbol '..' to auto complete (sort of like javascript arrays)
- Edit specific fields, copy/move the rest
- Note that the struct update syntax uses = like an assignment; this is because it moves the data
- Note in rust - these String count as moves - obviously copy trait for primitive data 
- Note though - this does invalidate user1 because we are moving the complex data of String (or other complex data)
    - In this case it's the user1.username that is the string data moved, forcing rust to drop the user1.username string.
- NOTE: the whole original struct becomes unusable as a value if the update syntax moves even one non-Copy field, but individual fields that weren't moved or just copied can still be accessed.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### Creating Different Types with Tuple Structs

Rust also supports structs that look similar to tuples, called tuple structs.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

-  Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
- Each struct you define is its own type, even though the fields within the struct might have the same types. 
    - For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. 
- Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value. 
- **Unlike tuples, tuple structs require you to name the type of the struct when you destructure them. For example, we would write let Point(x, y, z) = origin; to destructure the values in the origin point into variables named x, y, and z.**

### Defining Unit-Like Structs

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section.

- *I remember we used these like flags in the Game Engine development course for not functional attributes in ECS*

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

- No need for curly brackets or parentheses. 
Then, we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. 
    - Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. 

## Ownership of Struct Data

In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

- It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes
    - to be discussed later
- Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is
Let’s say you try to store a reference in a struct without specifying lifetimes, like the following
```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
- The compiler will complain that it needs lifetime specifiers
    - error[E0106]: missing lifetime specifier

## 5.2 An Example Program Using Structs

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
 - *Effectively showing a purpose of interface:*
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
- We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. 
- We don’t want dbg! to take ownership of rect1 (because there is no expression to change value), so we use a reference to rect1 in the next call.

Other traits are listed in [Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)


## 5.3 Methods

Methods are similar to functions: We declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. *pretty standard stuff*

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- Not the impl keyword, then define functions normally - except with self reference (for readable) to access fields.
    - We'd used &mut self normally to write still.
    -  Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
- value is organization - and intuition (won't elaborate - it's boring)

variation
```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
- note: ' self.width > 0' we are subtly adding condition - so it won't print if less that 0.
- Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called getters.
    - Rust does not implement them automatically for struct fields as some other languages do. 

### Where’s the -> Operator?

Rust doesn’t have an equivalent to the -> operator (in C/C++); instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.

- Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method.

In other words, the following are the same:
```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

**The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.**

### Methods with More Parameters
```rust

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### Associated Functions
All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. **We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.** We’ve already used one function like this: the String::from function that’s defined on the String type.

-Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
    // let rect4 = Rectangle::square(100);
```

The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
- To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example. This function is namespaced by the struct: The :: syntax is used for both associated functions and namespaces created by modules. We’ll discuss modules in 

### Multiple impl Blocks

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
- We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.

## Summary

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.