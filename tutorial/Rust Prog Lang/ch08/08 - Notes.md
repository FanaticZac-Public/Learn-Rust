# Chapter 08 - Common Collections

_now the good stuff_

Rust’s standard library includes a number of very useful data structures called collections. - collections can contain multiple values

- Unlike the built-in array and tuple types, the data that these collections point to is stored on the heap,
  - data does not need to be known at compile time and can grow or shrink as the program runs.
- Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill
  In this chapter, we’ll discuss three collections that are used very often in Rust programs::
- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter, we’ll talk about it in depth.
- A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.

## 8.1 Storing Lists of Values with Vectors

The first collection type we’ll look at is Vec<T>, also known as a vector.

- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. - Vectors can only store values of the same type.
- They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

### Creating a New Vector

```rust
   let v: Vec<i32> = Vec::new();
```

- add a type annotation as is typed and empty
- Vectors are implemented using generics
  More often, you’ll create a Vec<T> with initial values, and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.
- **Rust** conveniently provides the vec! macro, which will create a new vector that holds the values you give it.

```rust
  let v = vec![1, 2, 3];
```

- Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary.

### Updating a Vector

To create a vector and then add elements to it, we can use the push method

```rust
  let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

- As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword
- The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.
- _It doesn't state it but i guess making it mut and then pushing all same types later means compiler won't complain unless there is another type added._

### Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or by using the get method.

- Below annotated the types of the values that are returned from these functions for extra clarity.

```rust
   let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

- Using & and [] gives us a reference to the element at the index value.
- When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.

Rust provides these two ways to reference an element so that you can choose how the program behaves when you try to use an index value outside the range of existing elements.

- As an example, let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique

```rust
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // BAD - VERY  BAD! PANIC!
    let does_not_exist = v.get(100);
```

- When we run this code, the first [] method will cause the program to panic because it references a nonexistent element.
- When the get method is passed an index that is outside the vector, it returns None without panicking.
  - _This is reiteration of notes but very important - RUST DO NOT_ but showcases the Rust version of null checks that avoid randdom memory access and handles all outcomes
  - In short, use .get() access whenever possible for safety
- When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure that this reference and any other references to the contents of the vector remain valid.
  - Recall the rule that states you can’t have mutable and immutable references in the same scope.
    That rule applies in below, where we hold an immutable reference to the first element in a vector and try to add an element to the end. This program won’t work if we also try to refer to that element later in the function.

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
```

- error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
- Why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work:
  - Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
  - In that case, the reference to the first element would be pointing to deallocated memory.
  - The borrowing rules prevent programs from ending up in that situation.
- _In short - we cannot ask for a reference to a vector element in the same scope we are modifying the vector._ but could in a loop or some minor scope where the reference will be destroyed before modification occurs.

### Iterating Over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.

- how to use a for loop to get immutable references to each element in a vector of i32 values and print them.

```rust
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
```

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.

```rust
   let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
```

- So is mutable then and mut is put on Vector and in mutable reference of for-loop
  - The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.
- To change the value that the mutable reference refers to, we have to use the \* dereference operator to get to the value in i before we can use the += operator. (more on reference to value ch15)
- Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker’s rules. If we attempted to insert or remove items in the for loop bodies we would get a compiler error

### Using an Enum to Store Multiple Types

Use enums to overcome the same type limitation of Vectors

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

- _Same typezies_
- Rust needs to know what types will be in the vector at compile time so that it knows exactly how much memory on the heap will be needed to store each element.
- We must also be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.
- Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled
- _I was wondering then what it's allocating on the heap - but it's just the enum which would be limited in size - basically holding address to another piece of memory so still valid even if data-types are difference sizes and locations_
- If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work.
  - Instead, you can use a trait object, which we’ll cover in Chapter 18.

### Dropping a Vector Drops Its Elements

Like any other struct, a vector is freed when it goes out of scope

```rust
   {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

- When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

## 8.2. Storing UTF-8 Encoded Text with Strings

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

- _Note - so string literals are then in the binary, represent a string slice - and therefore are not growable, not mutable, and not owned... FYI_ - so need to be converted to_string for those functions

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
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
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

## 8.3. Storing Keys with Associated Values in Hash Maps

The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.

- Many programming languages support this kind of data structure, but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.
- Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

### Creating a New Hash Map

One way to create an empty hash map is to use new and to add elements with insert.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

### Accessing Values in a Hash Map

We can get a value out of the hash map by providing its key to the get method

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
```

- Here, score will have the value that’s associated with the Blue team, and the result will be 10. The get method returns an Option<&V>
- if there’s no value for that key in the hash map, get will return None
- This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>
- then unwrap_or to set score to zero if scores doesn’t have an entry for the key.
  We can iterate over each key-value pair in a hash map in a similar manner as we do with vectors, using a for loop:

```rust
 use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

### Managing Ownership in Hash Maps

- For types that implement the Copy trait, like i32, the values are copied into the hash map.
- For owned values like String, the values will be moved and the hash map will be the owner of those values
- If we insert references to values into the hash map, the values won’t be moved into the hash map.
  - The values that the references point to must be valid for at least as long as the hash map is valid

### Updating a Hash Map

- Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa)
  When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.
- You could replace the old value with the new value, completely disregarding the old value.
- You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
- Or you could combine the old value and the new value.

#### Overwriting a Value (Default)

If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
```

- This code will print {"Blue": 25}. The original value of 10 has been overwritten.

#### Adding a Key and Value Only If a Key Isn’t Present

Hash maps have a special API for this called entry that takes the key you want to check as a parameter.

- The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
- Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the entry API

```rust
   use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
```

- The 'or_insert' method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.

#### Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

- shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value

```rust
   use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // This code will print {"world": 2, "hello": 1, "wonderful": 1}
```

- The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
- Here, we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (\*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

### Hashing Functions

By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables1. - _Check out that later_

- This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
- If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
- A hasher is a type that implements the BuildHasher trait.

## Summary

Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.

Here are some exercises you should now be equipped to solve:

- Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
- Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
- Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
