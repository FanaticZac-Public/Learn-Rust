
<style>
@import url("../notes.css");
</style>


# Generic Types, Traits, and Lifetimes

Generics are abstract stand-ins for concrete types or other properties to reduce code duplication.

Functions can take parameters of some generic type, instead of a concrete type like i32 or String.

- Examples we've seen already are:
  - Option<T>
  - Vec<T>
  - HashMap<K, V>
  - Result<T, E>

Chapter discuss same techniques, making your own, how to use traits (to only accept certain behaviors), and lifelines feature.

lifetimes: a variety of generics that give the compiler information about how references relate to each other. Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure that references will be valid in more situations than it could without our help.

### Removing Duplication by Extracting a Function

## 10.1 Generic Data Types

### In Function Definitions
### In Struct Definitions
### In Enum Definitions
### In Method Definitions
### Performance of Code Using Generics

## 10.2 Defining Shared Behavior with Traits

### Defining a Trait
### Implementing a Trait on a Type
### Using Default Implementations
### Using Traits as Parameters
### Returning Types That Implement Traits
### Using Trait Bounds to Conditionally Implement Methods

## 10.3 Validating References with Lifetimes

### Dangling References
### The Borrow Checker
### Generic Lifetimes in Functions
### Lifetime Annotation Syntax
### In Function Signatures
### Relationships
### In Struct Definitions
### Lifetime Elision
### In Method Definitions
### The Static Lifetime
###  Generic Type Parameters, Trait Bounds, and Lifetimes

## Summary