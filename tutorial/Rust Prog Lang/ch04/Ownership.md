# Ownership

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

//-------------- String --------------
// Can't be mutated
let a = String::from("hello");

// Can be mutated - below
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{s}"); // this will print `hello, world!`


//-------------- When memory is dropped --------------
// It's dropped when it goes out of scope
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s

} // this scope is now over, and s is no longer valid


//-------------- Primitive type copy --------------
// primitive types stay on the stack so copying is cheap and easy
let x = 5;
let y = x; // both variables still good. (it's a copy but free performance wise)
// -- same for char, bool, int, float


//-------------- Complex Type copy--------------

// Complex types are on the heap
let s1 = String::from("hello"); // Allocated to heap
let s2 = s1; // This copy is a shallow copy and is default in Rust, preferred.
// New reference is added to stack - but it points to the same heap.
// IN RUST - s1 become invalided and removed to prevent double pointer/memory related bugs
println!("{s1}, world!");
// error[E0382]: borrow of moved value: `s1`


//-------------- Complex Type reassignment (called Move) --------------
    let mut s = String::from("hello"); // Note mutable
    s = String::from("ahoy"); // old s on heap gets dropped and pointer is pointed to new heap

    println!("{s}, world!");
    // ahoy, world!


//-------------- Explicit Deep-copy Complex Type --------------
    //  - copy stack and heap data
    // -- Very expensive performance - AVOID
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");


//-------------- Tuples and the Copy Trait --------------
// If a tuple is made up of primitives only (who only have the copy trait (related to stack) - and not the drop trait (related to heap)) - then the tuple will copy implicitly to the stack.
// - If even one of the members of tuple is complex - the whole thing will be treated like complex type


//-------------- Ownership and functions --------------
// Same as previous rules copy or move based on type - when passed as parameter.
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


//-------------- Return Values and Scope --------------
// Returning values can also transfer ownership.
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
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
   <strong>And below is my original note summary.
</strong><br>Still more condensed then the book - but more verbose then all the code examples with annotations
    </td>
  </tr>
</table>

## What Is Ownership?

Ownership is a set of rules that govern how a Rust program manages memory. It's one of the features that make Rust unique.

This is usually handled with

- The programmer explicitly allocating and freeing memory
- a programming language with garbage collection that regularly looks for no-longer-used memory as the program runs.

Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks.

- If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

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
   <strong>Awesome - here we go baby!
</strong>
    </td>
  </tr>
</table>

Examples will focus on a very common data structure: strings.

## Ownership Rules - Very Important!

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope

- Just talks about normal scope (Read the doc if you want) - from every language practically

## The String Type

Basic data types **will be on the stack**

- e.g. int, float, char, bool,
- Can be quickly and trivially copied to make a new independent instance if value is needed in new scope

We've already seen string literals `let s = "hello";` which are hardcoded into our program, convenient but not versatile.

- They are immutable also.
- Aren't always known at compile time.
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
   <strong>Fun fact - string literals aren't on the stack or heap
</strong><br> They're directly in the binary code - read only
    </td>
  </tr>
</table>

String **will be on the heap**

- Strings can be modified while the program runs (unlike string literal)
  - aka string literals but be known at compile time.
- knowing how data is cleaned up is important is good use case for ownership
  - Also valid for other complex data types in the std library

```rust
// String type manages data allocation on the heap during program operation

// Can't be mutated
let a = String::from("hello");

// double colon :: operator allows us to namespace this particular from function under the String type.

// Can be mutated - below
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{s}"); // this will print `hello, world!`
```

## Memory and Allocation

In order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

- The memory must be requested from the memory allocator at runtime.
  - That first part is done by us: When we call String::from, its implementation requests the memory it needs (standard)
- We need a way of returning this memory to the allocator when we’re done with our String.
  - We need to pair exactly one allocate with exactly one free.
- **How Rust handles this is to automatically release memory when it goes out of scope.**
  - **When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.**

```rust
// Rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s

} // this scope is now over, and s is no longer valid
```

This pattern has a profound impact on the way Rust code is written. It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap. Let’s explore some of those situations now.

## Variables and Data Interacting with Move

```rust
    let x = 5;
    let y = x;
```

- Bind the value 5 to x; then, make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

```rust
    let s1 = String::from("hello");
    let s2 = s1;
```

- This looks very similar, so we might assume that the way it works would be the same: That is, the second line would make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens.
  ![String](https://doc.rust-lang.org/book/img/trpl04-01.svg)
- String has 3 parts, ptr to memory, len (bytes), capacity
- On left is the stack - right is the heap memory
- capacity is the total amount of memory that string has received from the allocator
- When we assign s1 to s2 - the string data is copied, meaning we copy the pointer, length and capacity that is on the stack. - We do not copy the heap
  ![String](https://doc.rust-lang.org/book/img/trpl04-02.svg)
- By doing this by reference - is better performance
- When one of these references goes out of scope - drop would automatically clear the heap causing "double free" error - memory saftey bug.
- \*\* To ensure memory safety - Rust considers s1 no longer valid - preventing this error

```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
    // error[E0382]: borrow of moved value: `s1`
```

- This is neither a shallow copy or deep copy
  - the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy
  - And if it copied the heap also - that would be a deep copy
- Because in Rust the first variable is invalidated - it's called a move
- This solves the memory problem
- **Rust will never automatically create "deep" copies - by design choice**
  - Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance

### Scope and Assignment

The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well.

- When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.

```rust
    let mut s = String::from("hello");
    s = String::from("ahoy"); // old s on heap gets dropped and pointer is pointed to new heap

    println!("{s}, world!");
    // ahoy, world!
```

### Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called 'clone'

- This visual indicator is good (no deep copy without it) and represents and expensive execution
  If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

```rust
    // Explicit Deep copy - copy stack and heap data
    // -- Very expensive performance
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```

### Stack-Only Data: Copy

Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.

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
   <strong>In short - if an object has the copy trait - it's a primitive or made up of primitives, and if it has the drop trait, it or some part of it is on the heap.
</strong><br> and must obey the rules.
    </td>
  </tr>
</table>

What has the Copy Trait?
All the integer types, such as u32.
The Boolean type, bool, with values true and false.
All the floating-point types, such as f64.
The character type, char.
Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

- All the integer types, such as u32.
- The Boolean type, bool, with values true and false.
- All the floating-point types, such as f64.
- The character type, char.
- **Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.**

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

```rust

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

- So using String or any complex type will not have the copy trait for the stack - so passing value to a function puts it into new scope and takes ownership - invalidating the variable - and then at end of scope of that function calls drop
- But passing primitive type will just do a copy because its cheap and safe anyway - same as variable assignment

## Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
