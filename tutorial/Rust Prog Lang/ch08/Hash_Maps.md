# Storing Keys with Associated Values in Hash Maps

## Super Summary
```rust
//-------------- Creating a New Hash Map --------------
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);


//-------------- Accessing Values in a Hash Map --------------
// get method
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
// 10

// for loop
for (key, value) in &scores {
    println!("{key}: {value}");
}
// Yellow: 50
// Blue: 10


//-------------- Managing Ownership in Hash Maps --------------
// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved and the hash map will be the owner of those values
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point - they are owned by the hash map

// If we insert references to values into the hash map, the values won’t be moved into the hash map. 

//-------------- Updating a Hash Map --------------
// Overwriting a Value
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}");

// Adding a Key and Value Only If a Key Isn’t Present
// - Note .entry().or_insert() - rather than insert()
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50); 

// Updating a Value Based on the Old Value
let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
{"hello": 1, "wonderful": 1, "world": 2}


//-------------- Hashing Functions --------------
// By default, HashMap uses a hashing function called SipHash that can provide resistance to denial-of-service (DoS) attacks involving hash tables1.
// This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. 
// If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. (more on crates.io)
```

## Storing Keys with Associated Values in Hash Maps

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