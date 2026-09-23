# Chapter 04 - Understanding Ownership

Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

# 4.1 - What is Ownership?

Ownership is a set of rules that govern how a Rust program manages memory.

- Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. 
    - Rather than garbage collection or manually allocating and freeing the memory.
-  If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
    - Awesome - here we go baby!

 In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.

## The Stack and the Heap
In a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions. (Good review so ill keep some in)
- Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out (LIFO). 
    - Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size. 
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
- The heap is less organized: When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
- This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
    - Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.
- Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you.
- Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. 
    - Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process.
- By the same token, a processor can usually do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).
- When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.
- Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often. But knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## Ownership Rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Variable Scope
- Just talks about normal scope

## The String Type
- Basic data type covered already will be on the stack 
    - Can be quickly and trivally copied to make a new independant instance if value is needed in new scope
- String will be on the heap - knowing how that data is is cleaned up is important is good use case for ownership
    - Also valid for other complex data types
- We've already seen string literals '  let s = "hello"; ' which are convenient but aren't a versitile. 
    - They aren't immutable also. 
    - Aren't always known at compile time.
Rust as the String type:
```rust
let s = String::from("hello");
```
- The double colon :: operator allows us to namespace this particular from function under the String type.
Can be mutated
```rust
 let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // this will print `hello, world!`
```
- difference is how memory is used

## Memory and Allocation
in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
- The memory must be requested from the memory allocator at runtime.
    - That first part is done by us: When we call String::from, its implementation requests the memory it needs (standard)
- We need a way of returning this memory to the allocator when we’re done with our String.
    - ** This is unique - with no garbage collector how Rust handles this is to automatically release memory when it goes out of scope.
- ** When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
 - ** It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap. 
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
- When we assign s1 to s2 - the string data is copied, meaning we copy the pointer, length and capacity that is on the stack. 
    - We do not copy the heap
![String](https://doc.rust-lang.org/book/img/trpl04-02.svg)
- By doing this by reference - is better performance
- When one of these references goes out of scope - drop would automatically clear the heap causing "double free" error - memory saftey bug.
- ** To ensure memory safety - Rust considers s1 no longer valid - preventing this error
```rust
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
```
- this will throw a "error[E0382]: borrow of moved value: `s1`"
- This is neither a shallow copy or deep copy (familiar)
    - But because first variable is invalidated - it's called a move
- This solves the memory problem
- ** Rust will never automatically create "deep" copies - by design choice
    - Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime peformance

## Scope and Assignment
The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well. 
-  When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.

## Variables and Data Interacting with Clone
If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called 'clone'
- This visual indicator is good (no deep copy without it) and represents and expensive execution

## Stack-Only Data: Copy
```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```
- It's important to remember that this copy will still work on these primative types - because we know them at compile time and these are on the stack only - not the heap.
    - So there is no reason to not want to allow a copy and there is no deep or shallow version anyway - its just a copy
- Note: Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
- Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error. 
- So, what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:
    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating-point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

## Ownership and Functions
The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

- So using String or any complext type will not have the copy trait for the stack - so passing value to a function puts it into new scope and takes ownership - invaidating the variable - and then at end of scope of that function calls drop
- But passing primative type will just do a copy because its cheap and safe anyway - same as variable assignment

## Return Values and Scope
Returning values can also transfer ownership. Listing 4-4 shows an example of a function that returns some value, with similar annotations as those in Listing 4-3.
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
- So here it should be noted that these string raviables are being moved in all instances here because complex - but return does move
- While this works, taking ownership and then returning ownership with every function is a bit tedious. 
- Use tuples to give your parameter and return both parameter and returned value if needed
```rust 
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

- while this works - it's still a bit tedius - so we can use references... (why the lead up like this...)
# 4.2 - References and Borrowing
We can provide a reference to the String value. A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. 
- Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
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
- Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.
- Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.
- Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:
- So reference will be dropped by end of scope without affecting the original variable
    - there is no drop becuase no ownership - it's just unstacked
- creating a refence is called 'borrowing'
- borrowing something will not allow you to change it - because you don't have ownership.
- just as variables are immutable - so is references
- Note - we can create a variable in the function and simply return it consituting a move
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
    - this restrictoin prevents race conditions that happen when these 3 things happen
        - Two or more pointers access the same data at the same time.
        - At least one of the pointers is being used to write to the data.
        - There’s no mechanism being used to synchronize access to the data.
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
- So this will be allowed because we are not calling the mimmutable references after println
- compilers will basically just check whether immutable references are used after a mutable one is created in the same scope.
- These have huge safety advantages with Rust

## Dangling References
In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
- In Rust, by contrast, the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
-Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:
(example is in code - its not relevent as it's caught with compiler in rust) error[E0106]: missing lifetime specifier
- This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem: "this function's return type contains a borrowed value, but there is no value
for it to be borrowed from"

## The Rules of References

Let’s recap what we’ve discussed about references:

    - At any given time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.

# 4.3 - The Slice Type
This was the longest section in the book but to say that we want a reference to a substring of the string... 

```rust
    let s = String::from("hello world");

    let hello = &s[0..5]; // can drop the 0 [..5]; is same for first index
    let world = &s[6..11]; // or last [6..];

    // or you can use numeric variable 
    let len = s.len();
    let slice = &s[3..len];

    // or take whole thing
    let slice = &s[..];d
```
![Slice Type](https://doc.rust-lang.org/book/img/trpl04-07.svg)

- Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.
- Ok - but the value of ranges with sliced references is that if the original String is removed - if we were just returning the index instead of the Sliced references it wouldn't technically cause an error. 

Danger (use slice instead to avoid used indexes to potentiall dropped variable):
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

## String Literals as Slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:
```rust
let s = "Hello, world!";
```
- The type of s here is &str: It’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

## String Slices as Parameters

Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

```rust
fn first_word(s: &String) -> &str {}
```

A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

```rust
fn first_word(s: &str) -> &str {}
```
If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions, a feature we will cover in the “Using Deref Coercions in Functions and Methods” section of Chapter 15.

- Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

## Other Slices
```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```
-- it works for arrays - i get it.

## Summary

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages. But having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.