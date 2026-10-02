# Treating Smart Pointers Like Regular References

## Super Summary
```rust
//-------------- Following the Reference to the Value --------------
fn main() {
    // we create a reference to an i32 value and then use the dereference operator to follow the reference to the value.
    let x = 5;
    let y = &x;

    assert_eq!(5, x); // true
    assert_eq!(5, *y); // true - y point to x reference and needs be dereferenced to get value.

    assert_eq!(5, y);
    // error[E0277]: can't compare `{integer}` with `&{integer}`
}

//-------------- Using Box<T> Like a Reference --------------
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // samezies - in effect
}

//-------------- Defining Our Own Smart Pointer --------------
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// This won't compile because Rust doesn’t know how to dereference MyBox.
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// error[E0614]: type `MyBox<{integer}>` cannot be dereferenced


//-------------- Implementing the Deref Trait --------------
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // implementation for trait Deref
    // - method named deref that borrows self and returns a reference to the inner data.
    fn deref(&self) -> &Self::Target {
        &self.0 // .0 accesses the first value in a tuple struct. 
    }
}

// this works now
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Rust actually runs this: *(y.deref()) 
}

//-------------- Using Deref Coercion in Functions and Methods --------------
// Deref coercion - converts a reference to a type that implements the Deref trait, into a reference to another type. 
// - For example, deref coercion can convert &String to &str 
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // MyBox defines deref() because it has the Deref trait
    let m = MyBox::new(String::from("Rust")); 
    hello(&m); // Deference returns &String and again for &str
    // hello(&(*m)[..]); - is how syntax would look otherwise
}
// Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type.
// The number of times that Deref::deref needs to be inserted is resolved at compile time - no runtime penalty for taking advantage of deref coercion!

//-------------- Handling Deref Coercion with Mutable References --------------
// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

// Just does deref coercion when it finds types and trait implementations in three cases:

// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>

```
### Definitions and Terms from chapter
- *Deref coercion* - converts a reference to a type that implements the Deref trait into a reference to another type. 

## Treating Smart Pointers Like Regular References

Implementing the Deref trait allows you to customize the behavior of the dereference operator \* (not to be confused with the multiplication or glob operator).

- By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

- Let’s first look at how the dereference operator works with regular references.
- Then, we’ll try to define a custom type that behaves like Box<T> and see why the dereference operator doesn’t work like a reference on our newly defined type.
- We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references.
- Then, we’ll look at Rust’s deref coercion feature and how it lets us work with either references or smart pointers.

### Following the Reference to the Value

A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// we create a reference to an i32 value and then use the dereference operator to follow the reference to the value.
```

- Comparing a number and a reference to a number isn’t allowed because they’re different types.
- We must use the dereference operator to follow the reference to the value it’s pointing to.

## Using Box<T> Like a Reference

- We can rewrite the code to use a Box<T> instead of a reference; the dereference operator used on the Box<T> functions in the same way as the dereference operator used on the reference.
```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
- The main difference is that here we set y to be an instance of a box pointing to a copied value of x rather than a reference pointing to the value of x.
- In the last assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when y was a reference.

## Defining Our Own Smart Pointer

Let’s build a wrapper type similar to the Box<T> type provided by the standard library to experience how smart pointer types behave differently from references by default. 

- Note: There’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: Our version will not store its data on the heap. 
    - We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.

The Box<T> type is ultimately defined as a tuple struct with one element, MyBox<T> type in the same way. 
    - We’ll also define a new function to match the new function defined on Box<T>.

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```
- We define a struct named MyBox and declare a generic parameter T because we want our type to hold values of any type.
- The MyBox type is a tuple struct with one element of type T. 
- The MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.

Let’s try adding the main function in Listing 15-7 to Listing 15-8 and changing it to use the MyBox<T> type we’ve defined instead of Box<T>. 
    - The code in Listing 15-9 won’t compile, because Rust doesn’t know how to dereference MyBox.

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

- Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. 
- To enable dereferencing with the * operator, we implement the Deref trait.

## Implementing the Deref Trait

- As discussed in “Implementing a Trait on a Type” in Chapter 10, to implement a trait we need to provide implementations for the trait’s required methods.
- The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data. 

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    // implementation for trait Deref
    // - method named deref that borrows self and returns a reference to the inner data.
    fn deref(&self) -> &Self::Target {
        &self.0 // .0 accesses the first value in a tuple struct. 
    }
}

// this works now
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Rust actually runs this: *(y.deref()) 
}
```
- Without the Deref trait, the compiler can only dereference & references. 
- The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a reference that it knows how to dereference.
- The reason the deref method returns a reference to a value, and that the plain dereference outside the parentheses in *(y.deref()) is still necessary, has to do with the ownership system. 
- If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self.
- We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.

Note that the * operator is replaced with a call to the deref method and then a call to the * operator just once, each time we use a * in our code. 
- Because the substitution of the * operator does not recurse infinitely, we end up with data of type i32, which matches the 5 in assert_eq!

## Using Deref Coercion in Functions and Methods
- Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. 
- For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. 
- Deref coercion is a convenience Rust performs on arguments to functions and methods, and it works only on types that implement the Deref trait. 
- It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
- A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

- Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *. 
- The deref coercion feature also lets us write more code that can work for either references or smart pointers.

```rust
// To see deref coercion in action, let’s use the MyBox<T>
fn hello(name: &str) {
    println!("Hello, {name}!");
}
// We can call the hello function with a string slice as an argument, such as hello("Rust");, for example. Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);// Deference returns &String and again for &str
    // hello(&(*m)[..]); - is how syntax would look otherwise
}
```

- Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type.
- The number of times that Deref::deref needs to be inserted is resolved at compile time - no runtime penalty for taking advantage of deref coercion!

## Handling Deref Coercion with Mutable References

Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Just does deref coercion when it finds types and trait implementations in three cases:

1. From &T to &U when T: Deref<Target=U>
2. From &mut T to &mut U when T: DerefMut<Target=U>
3. From &mut T to &U when T: Deref<Target=U>

- The first two cases are the same except that the second implements mutability. 
    - he first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently. 
- The second case states that the same deref coercion happens for mutable references.
- The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
- But the reverse is not possible: Immutable references will never coerce to mutable references.
- Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
- Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
- Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.