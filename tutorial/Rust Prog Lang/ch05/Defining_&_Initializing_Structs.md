# Defining and Instantiating Structs

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

//-------------- Struct Usage --------------
// Definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Creating Instance
// !! Rust doesn’t allow us to mark only certain fields as mutable.
let mut user1 = User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

// Accessing (Dot Notation) - mutable struct so we can change field
user1.email = String::from("anotheremail@example.com");

// returning
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Field initialization shorthand - vs ^^
// - You don't have to repeat field names email:emails, usernames:username
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


//-------------- Struct Update Syntax --------------
// Wanting to copy most of existing struct into new instance?
// -- Move to new instance - keeping some fields
// Note that the struct update syntax uses = like an assignment; this is because it moves the data (user1 becomes invalid)
// - Because the complex data is moved (if it was just primitives, would be a copy).

// regular way
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};

// Struct Update Syntax (using '..' syntax)
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
// Since we didn't move email, created new, user1.email is still accessible - but user1.username has been moved.

//-------------- Tuple Structs --------------
// Like tuples but with more meaningful name, no field names

// Each struct you define is its own type
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//-------------- Unit-Like Structs --------------
struct AlwaysEqual;
let subject = AlwaysEqual;

// In games - we used these like flags. Say a player could have user or enemy flag - but have all same behaviors. But you might want to filter by it during the game loop


//-------------- Ownership of Struct Data --------------
// - we used the owned String type rather than the &str string slice type before because we wanted struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
struct User {
    active: bool,
    username: &str, // ref
    email: &str, // ref
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
// error[E0106]: missing lifetime specifier
// - It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes (ch10)

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

## Defining and Instantiating Structs

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

- Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
- Each struct you define is its own type, even though the fields within the struct might have the same types.
  - For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.
- Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
- **Unlike tuples, tuple structs require you to name the type of the struct when you destructure them. For example, we would write let Point(x, y, z) = origin; to destructure the values in the origin point into variables named x, y, and z.**

### Defining Unit-Like Structs

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section.

- _I remember we used these like flags in the Game Engine development course for not functional attributes in ECS_

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

- No need for curly brackets or parentheses.
  Then, we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses. - Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes.

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
