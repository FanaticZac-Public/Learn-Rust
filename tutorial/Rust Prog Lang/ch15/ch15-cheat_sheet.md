# Smart Pointers

A pointer is a general concept for a variable that contains an address in memory. 

This address refers to, or “points at,” some other data. 

The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4. 

References are indicated by the & symbol and borrow the value they point to. 

They don’t have any special capabilities other than referring to data, and they have no overhead.

Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities. 

The concept of smart pointers isn’t unique to Rust: Smart pointers originated in C++ and exist in other languages as well. 

Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references. 

To explore the general concept, we’ll look at a couple of different examples of smart pointers, including a reference counting smart pointer type. 

This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.

In Rust, with its concept of ownership and borrowing, there is an additional difference between references and smart pointers: While references only borrow data, in many cases smart pointers own the data they point to.

Smart pointers are usually implemented using structs. 

Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. 

The Deref trait allows an instance of the smart pointer struct to behave like a reference so that you can write your code to work with either references or smart pointers. 

The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope. 

Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won’t cover every existing smart pointer. 

Many libraries have their own smart pointers, and you can even write your own. We’ll cover the most common smart pointers in the standard library:

- Box<T>, for allocating values on the heap
- Rc<T>, a reference counting type that enables multiple ownership
- Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API for mutating an interior value. 

We’ll also discuss reference cycles: how they can leak memory and how to prevent them.

Chapter 14 discusses:

- [Using Box<T> to Point to Data on the Heap](./Box<T>_To_Point_To_Heap_Data.md)
- [Treating Smart Pointers Like Regular References](./Treating_Smart_Pointers_Like_Regular_Refs.md)
- [Running Code on Cleanup with the Drop Trait](./Drop_Trait_Clean_Up.md)
- [Rc<T>, the Reference-Counted Smart Pointer](./Rc<T>_Reference-Counted_Smart_Pointer.md)
- [RefCell<T> and the Interior Mutability Pattern](./RefCell<T>_Interior_Mutability_Pattern.md)
- [Reference Cycles Can Leak Memory](./Ref_Cycles_Can_Leak_Memory.md)

## Recommended by chapter
- In Rust - Some properties of code are impossible to detect by analyzing the code: The most famous example is the Halting Problem, which is beyond the scope of this book but is an interesting topic to research.

## Post Summary

This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. The Box<T> type has a known size and points to data allocated on the heap. The Rc<T> type keeps track of the number of references to data on the heap so that the data can have multiple owners. The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers. We explored reference cycles that can cause memory leaks and how to prevent them using Weak<T>.

If this chapter has piqued your interest and you want to implement your own smart pointers, check out “The Rustonomicon” for more useful information.

