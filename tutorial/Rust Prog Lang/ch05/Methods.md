# Methods

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

## Super Summary

```rust
//-------------- Method Syntax --------------
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl (implementation) block for Rectangle
// - Everything within this impl block will be associated with the Rectangle type
// - Note the self reference field referring to struct
impl Rectangle {
    // method 1
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // method 2
    fn width(&self) -> bool {
        self.width > 0
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

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}


//-------------- Where’s the -> Operator? --------------
// Rust has a feature called 'automatic referencing and dereferencing'.
// - Calling methods is one of the few places in Rust with this behavior.
// When you call a method with object.something(), Rust automatically adds in &, &mut, or * so that object matches the signature of the method.
//  In other words, the following are the same:
p1.distance(&p2); // cleaner
(&p1).distance(&p2);

// This automatic referencing behavior works because methods have a clear receiver—the type of self.
// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).


//-------------- Methods with More Parameters --------------
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // an immutable borrow of other rectangle
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
    // Can rect1 hold rect2? true
    // Can rect1 hold rect3? false
}


//-------------- Associated Functions --------------
// All functions defined with impl are associated functions.
// However, we can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new, but new isn’t a special name and isn’t built into the language.

// To call this associated function, we use the :: syntax with the struct name;
let sq = Rectangle::square(3);


//-------------- Multiple impl Blocks --------------
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
// legal - you might want to seperate out for methods, and constructors, or traits.
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

## Methods

Methods are similar to functions: We declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. _pretty standard stuff_

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
  - Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
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

- Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or \* so that object matches the signature of the method.

In other words, the following are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

\*\*The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a

## 5.3 Methods

Methods are similar to functions: We declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. _pretty standard stuff_

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
  - Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
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

- Here’s how it works: When you call a method with object.something(), Rust automatically adds in &, &mut, or \* so that object matches the signature of the method.

In other words, the following are the same:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

**The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name obig part of making ownership ergonomic in practice.**

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
