# Iterators in Rust

This cheat sheet section is made up of 2 sub-sections of Chapter 13 (Functional Language Features: Iterators and Closures) of The Rust Programming Language
- Processing a Series of Items with Iterators
- Performance in Loops vs. Iterators

## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up. 

For example, the code in Listing 13-10 creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>. This code by itself doesn’t do anything useful.

```rust
    let v1 = vec![1, 2, 3];

    // The iterator is stored in the v1_iter variable  
    // Once we’ve created an iterator, we can use it in a variety of ways. 
    let v1_iter = v1.iter();

    // Previously used iterator on array using for loop under the hood.

    // We we have the iterator separated out, and iterate over each element
    for val in v1_iter {
        println!("Got: {val}");
    }

    // Iterators cut down the repetative code you could potentially mess up (aka index out of bounds)
    // Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.
```
## Methods That Consume the Iterator
```rust
    // The Iterator Trait and the next Method
    // All iterators implement a trait named Iterator that is defined in the standard library. 
    pub trait Iterator {
    type Item;

    // Returns Option - aka some or None (if at end)
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
    }

    // Iterator deconstruction
    {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // must be mut f

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    // iterator must be mutable here as an iterator changes internal state that the iterator uses to keep track of where it is in the sequence
    // In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. 
    // Note that the values we get from the calls to next are immutable references to the values in the vector. 
```
The `iter` method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call `into_iter` instead. Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of iter.

## Methods That Consume the Iterator
Many different methods with default implementation of the [iterator (Trait)](https://doc.rust-lang.org/std/iter/trait.Iterator.html). Some of these methods call the next method in their definition, which is why you’re required to implement the next method when implementing the Iterator trait.

Methods that call next are called *consuming adapters* because calling them uses up the iterator. 

One example is the [sum](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) method:

```rust
  #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

## Methods That Produce Other Iterators
*Iterator adapters* are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
- Instead, they produce different iterators by changing some aspect of the original iterator.

Example of calling the iterator adapter method map, which takes a closure to call on each item as the items are iterated through. The map method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);

    // Produces Warning:
    // warning: unused `Map` that must be used

    // the closure we’ve specified never gets called. The warning reminds us why: Iterator adapters are lazy, and we need to consume the iterator here.

    // To fix this warning and consume the iterator, we’ll use the collect method that consumes the iterator and collects the resultant values into a collection data type

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

Because map takes a closure, we can specify any operation we want to perform on each item. This is a great example of how closures let you customize some behavior while reusing the iteration behavior that the Iterator trait provides.

You can chain multiple calls to iterator adapters to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to iterator adapters.

## Closures That Capture Their Environment

Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.

For this example, we’ll use the filter method that takes a closure. The closure gets an item from the iterator and returns a bool. If the closure returns true, the value will be included in the iteration produced by filter. If the closure returns false, the value won’t be included.

we use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size.

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// function takes ownership of a vector of shoes and a shoe size as parameters. 
// Returns a new iterator that only contains elements for which the closure returns true.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Ok so here we filter with closure in a lambda style function easily
    // into_iter to create an iterator that takes ownership of the vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    // Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```



## Other / Related Links
- [iterator (Trait)](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
    - [next (Function)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next)
    - [sum (Function)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
    - [map (Function)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
    - [filter (Function)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)