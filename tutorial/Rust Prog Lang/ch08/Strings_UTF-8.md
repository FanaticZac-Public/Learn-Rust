# Storing UTF-8 Encoded Text with Strings

## Super Summary

```rust
//-------------- Defining Strings --------------
let mut s = String::new();


let data = "initial contents"; // string literal

let s = data.to_string(); // object String initialized with string literal to gain access to functionality presumably (or to edit if mutable)

// The method also works on a literal directly:
let s = "initial contents".to_string();

// Using ::from()
let s = String::from("initial contents");

// Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שלום");
let hello = String::from("नमस्ते");
// -- etc --


//-------------- Updating a String --------------
// A String can grow in size and its contents can change

// Appending with push_str or push
let mut s = String::from("foo");
s.push_str("bar");
// foobar

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2); // push_str() is reference via string slice
println!("s2 is {s2}");
// s2 is bar


let mut s = String::from("lo");
s.push('l'); // push() single character (copy trait)
// lol


//-------------- Updating a String --------------
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
// Hello, world!

// The + operator uses the add method
fn add(self, s: &str) -> String {}
// - self is not a ref &self - meaning it's taking ownership and returning String here.

// Concat multiple strings
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
// tic-tac-toe
// or use format! macro:
let s = format!("{s1}-{s2}-{s3}");

//-------------- Indexing into Strings --------------
// Indexing Strings by index causes error in Rust
let s1 = String::from("hi");
let h = s1[0];
// error[E0277]: the type `str` cannot be indexed by `{integer}`

// Why?
// A String is a wrapper over a Vec<u8>
let hello = String::from("Hola");
// In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long.
// Each of these letters takes 1 byte when encoded in UTF-8.

let hello = String::from("Здравствуйте");
// But this one is 2 bytes per character in UTF8

// ** Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
// Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

//-------------- Bytes, Scalar Values, and Grapheme Clusters --------------
// Read the longer notes for this topic - this section is informative - not instructive. In short Rust doesn't allow indexing of Strings.

// ******** Everything below this is potentially dangerous --- and there are libraires on crates.io that can help ******** Use Caution
//-------------- Slicing Strings (highly risky) --------------
// Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

let hello = "Здравствуйте";

let s = &hello[0..4];
// Зд

// If we were to try to slice only part of a character’s bytes with something like:
let s = &hello[0..1]; // half of 2 byte char
// Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:
// thread 'main' panicked at src/main.rs:4:19:
// byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
// ** You should use caution when creating string slices with ranges, because doing so can crash your program.**


//-------------- Iterating Over Strings --------------
// The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.

// For individual Unicode scalar values, use the chars method.
for c in "Зд".chars() {
    println!("{c}");
}
// З
// д

// the bytes method returns each raw byte
for b in "Зд".bytes() {
    println!("{b}");
}
208 // first half of З
151 // second half of З
208
180
// But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
```

## Storing UTF-8 Encoded Text with Strings

New Rustaceans commonly get stuck on strings for a combination of three reasons:

- Rust’s propensity for exposing possible errors,
- strings being a more complicated data structure than many programmers give them credit for,
- and UTF-8.
  These factors combine in a way that can seem difficult when you’re coming from other programming languages.
- We discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.
- In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading.
- String is different from the other collections, namely, how indexing into a String is complicated by the differences between how people and computers interpret String data.

### Defining Strings

We’ll first define what we mean by the term string.

- Rust has only one string type in the core language, which is the string slice 'str' that is usually seen in its borrowed form, '&str'
- we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere.
  - "String literals", for example, are stored in the program’s binary and are therefore string slices.
- The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
- When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.
  - Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

### Creating a New String

Many of the same operations available with Vec<T> are available with String as well because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.

- An example of a function that works the same way with Vec<T> and String is the new function to create an instance

```rust
   let mut s = String::new();
```

- This line creates a new, empty string called s, into which we can then load data.
- to_string is used to add initial data
  - which is available on any type that implements the Display trait, as string literals do.

```rust
   let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
```

This code creates a string containing initial contents. We can also use the function String::from to create a String from a string literal.

- _Note - so string literals are then let mut s = String::new();in the binary, represent a string slice - and therefore are not growable, not mutable, and not owned... FYI_ - so need to be converted to_string for those functions

```rust
    let s = String::from("initial contents");
```

- Because strings are used for so many things, we can use many different generic APIs for strings
- In this case, String::from and to_string do the same thing, so which one you choose is a matter of style and readability.
- Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown in

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

All of these are valid String values.

### Updating a String

A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it. In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

#### Appending with push_str or push

We can grow a String by using the push_str method to append a string slice

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
```

After these two lines, s will contain foobar. The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter. For example, in the code in Listing 8-16, we want to be able to use s2 after appending its contents to s1.

```rust
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
```

If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!

- The push method takes a single character as a parameter and adds it to the String. Listing 8-17 adds the letter l to a String using the push method.

```rust
    let mut s = String::from("lo");
    s.push('l');
```

As a result, s will contain lol.

#### Concatenating with + or format!

Often, you’ll want to combine two existing strings. One way to do so is to use the + operator

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator. The + operator uses the add method, whose signature looks something like this:

```rust
fn add(self, s: &str) -> String {
```

In the standard library, you’ll see add defined using generics and associated types. Here, we’ve substituted in concrete types, which is what happens when we call this method with String values.

- First, s2 has an &, meaning that we’re adding a reference of the second string to the first string. This is because of the s parameter in the add function: We can only add a string slice to a String - we can’t add two String values together.
- But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add.
  - The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.
- Second, we can see in the signature that add takes ownership of self because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that.
- **So, although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies, but it isn’t; the implementation is more efficient than copying.** - _This is worth another look with regard to the ownership and inferring the meaning of the signature designations to determine details of operation._
  If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
```

At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on. For combining strings in more complicated ways, we can instead use the format! macro:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
```

- The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
- format! macro uses references so that this call doesn’t take ownership of any of its parameters.

### Indexing into Strings

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.

- Consider the invalid code:

```rust
    let s1 = String::from("hi");
    let h = s1[0];
```

- Compiling collections v0.1.0 (file:///projects/collections)
  error[E0277]: the type `str` cannot be indexed by `{integer}`
- The error tells the story: Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.
  - _my guess same index danger potential_

#### Internal Representation

- A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:

```rust
    let hello = String::from("Hola");
```

- In this case, len will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8.
- The following line, however, may surprise you (note that this string begins with the capital Cyrillic letter Ze, not the number 3):

```rust
    let hello = String::from("Здравствуйте");
```

- If you were asked how long the string is, you might say 12. In fact, Rust’s answer is 24: That’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

- You already know that answer will not be З, the first letter. When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Users generally don’t want the byte value returned, even if the string contains only Latin letters: If &"hi"[0] were valid code that returned the byte value, it would return 104, not h.
- The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

#### Bytes, Scalar Values, and Grapheme Clusters

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

- If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135You should use caution when creating string slices with ranges, because doing so can crash your program.]
```

- That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

```rust
['न', 'म', 'स', '्', 'त', 'े']
```

There are six char values here, but the fourth and sixth are not letters: They’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

```rust
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

- A final reason Rust doesn’t allow us to index into a String to get a character _is that indexing operations are expected to always take constant time (O(1))_. But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. - If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.
Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

- Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд
- If we were to try to slice only part of a character’s bytes with something like &hello[0..1], Rust would panic at runtime in the same way as if an invalid index were accessed in a vector: - thread 'main' panicked at src/main.rs:4:19:
  byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`

**You should use caution when creating string slices with ranges, because doing so can crash your program.**

### Iterating Over Strings

_The best way to operate on pieces of strings_ is to be explicit about whether you want characters or bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:

```rust
//chars
for c in "Зд".chars() {
    println!("{c}");
}
// З
// д


//bytes
for b in "Зд".bytes() {
    println!("{b}");
}
// 208
// 151
// 208
// 180

```

- But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
- Getting grapheme clusters from strings, as with the Devanagari script, is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.

### Handling the Complexities of Strings

Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data up front. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

- The good news is that the standard library offers a lot of functionality built off the String and &str types to help handle these complex situations correctly.
- Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.
