# References and Borrowing

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

## Super Summarize

```rust
//-------------- Working Reference Example --------------
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // pass reference - s1 is 'borrowed' meaning it won't be moved, and is immutable

    println!("The length of '{s1}' is {len}.");
    // The length of 'hello' is 5.
}

fn calculate_length(s: &String) -> usize {
    s.len() // uses borrowed value to return output of function.
}


//-------------- Mutable Reference --------------

// Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value.
// - this restriction prevents race conditions that happen when these 3 things happen
//     1. Two or more pointers access the same data at the same time.
//     2. At least one of the pointers is being used to write to the data.
//     3. There’s no mechanism being used to synchronize access to the data.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

// &mut makes it very clear that the change function will mutate the value it borrows.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// ---
// As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

// --- 
// Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. (the compiler counts - when enforcing the usage)
println!("{r1}, {r2}, and {r3}"); // scope ends here for all 3
// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable


//-------------- Dangling References --------------
// Rust prevents dangling references
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
// error[E0106]: missing lifetime specifier
// an exception to this rule, lifetimes are discussed (ch10)

// But of course you can move ownership in the dangle function instead:
fn no_dangle() -> String {
    let s = String::from("hello");

    s // ownerships of s is moved out via function return
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
   <strong>Now back to your regularly scheduled programming
</strong>
    </td>
  </tr>
</table>

## References and Borrowing

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.

- Unlike a pointer, **a reference is guaranteed to point to a valid value of a particular type for the life of that reference.**
  - Enforced by the compiler
- Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
/// The length of 'hello' is 5.
```

![Reference Example](https://doc.rust-lang.org/book/img/trpl04-06.svg)

- & added to denote reference
- is just a pointer to the original variable which has a pointer to the memory on the heap
- Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, `*`. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.
- Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:
- So reference `s` will be dropped by end of scope without affecting the original variable
  - there is no drop because no ownership - it's just unstacked
- creating a reference is called 'borrowing'
  - borrowing something will not allow you to change it - because you don't have ownership.
  - just as variables are immutable - so are references
    - error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
- Note - we can create a variable in the function and simply return it constituting a move

## Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- note mut on s and on change parameter
- This makes it very clear that the change function will mutate the value it borrows.
- Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value.
  - error[E0499]: cannot borrow `s` as mutable more than once at a time
  - this restriction prevents race conditions that happen when these 3 things happen
    1. Two or more pointers access the same data at the same time.
    2. At least one of the pointers is being used to write to the data.
    3. There’s no mechanism being used to synchronize access to the data.
  - Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!
    As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rust
 let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
```

- We also cannot have a mutable reference while we have an immutable one to the same value.
  - Users of an immutable reference don’t expect the value to suddenly change out from under them!
- multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
  Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

```rust
 let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
```

- So this will be allowed because we are not calling the immutable references after println
- compilers will basically just check whether immutable references are used after a mutable one is created in the same scope.
- These have huge safety advantages with Rust

## Dangling References

In languages with pointers, it’s easy to erroneously create a *dangling pointer*, aka pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.

- In Rust, by contrast, the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
  -Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:
  (example is in code - its not relevant as it's caught with compiler in rust) error[E0106]: missing lifetime specifier
- This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem: "this function's return type contains a borrowed value, but there is no value
  for it to be borrowed from"

## The Rules of References

Let’s recap what we’ve discussed about references:

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
