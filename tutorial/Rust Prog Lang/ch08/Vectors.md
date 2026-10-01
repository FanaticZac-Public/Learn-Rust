# Storing Lists of Values with Vectors

## Super Summary

```rust
//-------------- Define Vector --------------
let v: Vec<i32> = Vec::new(); // requires type
let v = vec![1, 2, 3]; // inferred but initial values


//-------------- Update Vector --------------
// As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

//-------------- Reading Elements of Vector (2 ways) --------------
let v = vec![1, 2, 3, 4, 5];

// way 1
let third: &i32 = &v[2]; // ref to index
println!("The third element is {third}");

// way 2 - this way avoids potential index bugs
let third: Option<&i32> = v.get(2); // & in type
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}

//-------------- Accessing invalid index--------------
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100]; // will panic
let does_not_exist = v.get(100); // Returns None without panic


//-------------- Ownership - borrow checker --------------
// - Recall the rule that states you can’t have mutable and immutable references in the same scope.
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6); //     - immutable borrow occurs here

println!("The first element is: {first}");
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
// Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored

//-------------- Iterating Over the Values in a Vector --------------
// get immutable references to each element in a vector of i32 values and print them.
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

// iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
// - we have to use the * dereference operator to get to the value in i before we can use the += operator. (ch15)

//-------------- Using an Enum to Store Multiple Types --------------
// Vectors can only store values that are of the same type so that it knows exactly how much memory on the heap will be needed
// - enums can bypass this rule (compiler will account largest required)
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


//-------------- Dropping a Vector Drops Its Elements --------------
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here

```

## Storing Lists of Values with Vectors

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
