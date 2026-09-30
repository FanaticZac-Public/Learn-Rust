# The Slice Type
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
//-------------- String Slices --------------
// A string slice is a reference to a contiguous sequence of the elements of a String
// [starting_index..ending_index]
// starting_index is the first position in the slice
// ending_index is one more than the last position in the slice

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// starting index to end
let len = s.len();
let slice = &s[3..len]; // to end
let slice = &s[3..]; // to end

// or take whole thing
let slice = &s[0..len];
let slice = &s[..];

// Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multi-byte character, your program will exit with an error.

//-------------- String Slices --------------
// Example gets first word, ' ' delimited with Rust's range syntax
fn first_word(s: &String) -> &str {
let bytes = s.as_bytes();

for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' { 
        return &s[0..i];
    }
}

&s[..] // returns whole thing if not returned early above
}


//-------------- String Literals as Slices --------------
// Recall string literals are stored inside the binary. That means we can just take a string slice directly referencing them.

let s = "Hello, world!";

// So rather than create a String with a reference like last example, we can change the parameter to &str
fn first_word(s: &str) -> &str {}

// Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}


//-------------- Other Slices --------------
// Rust's range syntax works on other arrays.

let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
// You’ll use this kind of slice for all sorts of other collections. 
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

## The Slice Type
Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.

In idiomatic Rust, functions do not take ownership of their arguments unless they need to, and the reasons for that will become clear as we keep going.

(Basically the book has super long explanation whey taking string slices is cumbersome and error prone) - so they have a convention:

## String Slices

```rust
    let s = String::from("hello world");

    let hello = &s[0..5]; // can drop the 0 [..5]; is same for first index
    let world = &s[6..11]; // or last [6..];

    // or you can use numeric variable 
    let len = s.len();
    let slice = &s[3..len];

    // or take whole thing
    let slice = &s[..];
```
![Slice Type](https://doc.rust-lang.org/book/img/trpl04-07.svg)

- Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multi-byte character, your program will exit with an error.
- Ok - but the value of ranges with sliced references is that if the original String is removed - if we were just returning the index instead of the Sliced references it wouldn't technically cause an error. 

Danger (use slice instead to avoid used indexes to potentially dropped variable):
```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```
- This will create compile error instead because it's tied to original value.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
 - This is the case where that could happen
```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {word}");
}
```
- error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable for the second - and we want errors in compiler
- Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. 
-The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point. Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.
- Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

### String Literals as Slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:
```rust
let s = "Hello, world!";
```
- The type of s here is &str: It’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

### String Slices as Parameters

Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

```rust
fn first_word(s: &String) -> &str {}
```

A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

```rust
fn first_word(s: &str) -> &str {}
```
If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions, a feature we will cover in the “[Using Deref Coercions in Functions and Methods](https://doc.rust-lang.org/book/ch15-02-deref.html#using-deref-coercions-in-functions-and-methods)” section of Chapter 15.

- Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

### Other Slices
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
You’ll use this kind of slice for all sorts of other collections. 

