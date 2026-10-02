# Using Box<T> to Point to Data on the Heap

## Super Summary

```rust
//-------------- Smart Pointers - Box<T> --------------
//-------------- Storing Data on the Heap --------------
fn main() {
    let b = Box::new(5); // 5 on the heap instead of stack
    println!("b = {b}");
}

//-------------- Enabling Recursive Types with Boxes --------------
// The Cons List recursive example
enum List {
    Cons(i32, List),
    Nil,
}

// usage
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
// error[E0072]: recursive type `List` has infinite size

// The Cons List recursive example - Fixed by using smart pointer to List instead of trying to hold it recursively.
// - On heap - recursive Lists is now conceptually stored side by side - rather than recursively (inside) - meaning Rust can understand how to size their memory allocations.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data.
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Boxes provide only the indirection and heap allocation
// - The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. 

```
<table width="100%">
  <tr>
    <td width="50%" align="center" valign="top">
      <img
        src="https://doc.rust-lang.org/book/img/trpl15-01.svg"
        width="100%"
        alt="Rust panics"
      >
      <br>
      <strong>Recursive - error infinite</strong>
    </td>
    <td width="50%" align="center" valign="top">
      <img
        src="https://doc.rust-lang.org/book/img/trpl15-02.svg"
        width="100%"
        alt="Rust does not compile"
      >
      <br>
      <strong>Smart Pointer - broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.</strong>
    </td>
  </tr>
</table>


## Using Box<T> to Point to Data on the Heap

- Boxes allow you to store data on the heap rather than the stack.  
    - What remains on the stack is the pointer to the heap data.
- Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    -  But they don’t have many extra capabilities either.
You’ll use them most often in these situations:

1. When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size
    - We’ll demonstrate the first situation in “Enabling Recursive Types with Boxes”.
2. When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so
    - In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap. 
3. When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type
    - The third case is known as a trait object, and “Using Trait Objects to Abstract over Shared Behavior” in Chapter 18 is devoted to that topic. So, what you learn here you’ll apply again in that section!

## Storing Data on the Heap

Before we discuss the heap storage use case for Box<T>, we’ll cover the syntax and how to interact with values stored within a Box<T>.

```rust
// how to use a box to store an i32 value on the heap.
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```
- Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated
- The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

## Enabling Recursive Types with Boxes

- A value of a recursive type can have another value of the same type as part of itself. 
- Recursive types pose an issue because Rust needs to know at compile time how much space a type takes up. 
- However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs.
- Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

As an example of a recursive type, let’s explore the cons list. 
- This is a data type commonly found in functional programming languages. 

### Understanding the Cons List

A cons list is a data structure that comes from the Lisp programming language and its dialects, is made up of nested pairs, and is the Lisp version of a linked list. 
-  Its name comes from the cons function (short for construct function) in Lisp that constructs a new pair from its two arguments.
- By calling cons on a pair consisting of a value and another pair, we can construct cons lists made up of recursive pairs.

For example, here’s a pseudocode representation of a cons list containing the list 1, 2, 3 with each pair in parentheses:

```
(1, (2, (3, Nil)))
```

- Each item in a cons list contains two elements: the value of the current item and of the next item. 
- The last item in the list contains only a value called Nil without a next item. 
- A cons list is produced by recursively calling the cons function. 
- The canonical name to denote the base case of the recursion is Nil.
- Note that this is not the same as the “null” or “nil” concept discussed in Chapter 6, which is an invalid or absent value.

The cons list isn’t a commonly used data structure in Rust. 
- Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use. 
- Other, more complex recursive data types are useful in various situations, but by starting with the cons list in this chapter, we can explore how boxes let us define a recursive data type without much distraction.

Contains an enum definition for a cons list. Note that this code won’t compile yet, because the List type doesn’t have a known size, which we’ll demonstrate.

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```
- Note: We’re implementing a cons list that holds only i32 values for the purposes of this example. We could have implemented it using generics, as we discussed in Chapter 10, to define a cons list type that could store values of any type.

Using the List type to store the list 1, 2, 3 would look like the code in

```rust
// --snip--

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
// error[E0072]: recursive type `List` has infinite size
```

- The first Cons value holds 1 and another List value.
- This List value is another Cons value that holds 2 and another List value. 
- This List value is one more Cons value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.
- error[E0072]: recursive type `List` has infinite size
- The error shows this type “has infinite size.”
- The reason is that we’ve defined List with a variant that is recursive
-  It holds another value of itself directly. 
- As a result, Rust can’t figure out how much space it needs to store a List value.
- We need to know how Rust decides how much space it needs to store a value of a non-recursive type.

### Computing the Size of a Non-Recursive Type

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
- To determine how much space to allocate for a Message value, Rust goes through each of the variants to see which variant needs the most space. 
Rust sees that Message::Quit doesn’t need any space, 
Message::Move needs enough space to store two i32 values
- Only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum in Listing 15-2 needs. 
- The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List. 
- Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List.
- To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant. 
- The Cons variant holds a value of type i32 and a value of type List, and this process continues infinitely

![An infinite List consisting of infinite Cons variants](https://doc.rust-lang.org/book/img/trpl15-01.svg)

### Getting a Recursive Type with a Known Size

Because Rust can’t figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:

```
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

- In this suggestion, *indirection* means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.
- Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: A pointer’s size doesn’t change based on the amount of data it’s pointing to.
- This means we can put a Box<T> inside the Cons variant instead of another List value directly. 
- The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
- Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.

We can change the definition of the List enum in Listing 15-2 and the usage of the List in Listing 15-3 to the code in Listing 15-5, which will compile.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

- The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
- The Nil variant stores no values, so it needs less space on the stack than the Cons variant.
- We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data.
- By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value. 

![A List that is not infinitely sized, because Cons holds a Box](https://doc.rust-lang.org/book/img/trpl15-02.svg)

- Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.
- They also don’t have the performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need. 
- The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. 
- When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. 