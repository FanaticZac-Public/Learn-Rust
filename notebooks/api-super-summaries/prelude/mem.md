# std::mem (Module mem ) - Quick Reference Methods (capabilities)

- [Module mem API](https://doc.rust-lang.org/beta/std/mem/index.htmll)


`std::mem` provides functions and types for working with values, layouts, and initialization. 

† marks nightly-only or experimental APIs;

 ⚠ marks unsafe APIs. 


### Functions

**Moving and dropping values**

- `drop` — Dispose of a value at a chosen point.
- `replace` — Put a new value in a mutable location and return the old value.
- `swap` — Exchange the values at two mutable locations.
- `take` — Replace a value with its default and return the old value.
- `forget` — Consume a value without running its destructor.
- `forget_unsized`† — Like `forget`, but also accepts unsized values.
- `needs_drop` — Check whether values of a type need destructor processing.

**Size, alignment, and layout**

- `size_of` — Return a type’s size in bytes.
- `size_of_val` — Return the size of a pointed-to value in bytes.
- `size_of_val_raw`†⚠ — Return the size of a value through a raw pointer.
- `align_of` — Return a type’s required alignment in bytes.
- `align_of_val` — Return the alignment of a pointed-to value in bytes.
- `align_of_val_raw`†⚠ — Return the alignment of a value through a raw pointer.
- `offset_of!` — Macro that returns a field’s byte offset within a type.

**Enums and type conversion**

- `discriminant` — Get a value identifying an enum’s variant, ignoring its fields.
- `variant_count`† — Return the number of variants in an enum type.
- `transmute`⚠ — Reinterpret a value’s bits as another type; both types must have the same size, and the result must be valid.
- `transmute_copy`⚠ — Copy and reinterpret a value through a reference as another type; the result must be valid.
- `transmute_neo`†⚠ — Experimental alternative to `transmute`.
- `transmute_prefix`†⚠ — Experimental conversion that initializes a destination from the common byte prefix of a source.

**Other experimental functions**

- `copy`† — Bitwise-copy a value.
- `conjure_zst`†⚠ — Create an instance of an inhabited zero-sized type.

### Types

- `ManuallyDrop<T>` — Wrap a value to prevent automatic destructor execution.
- `MaybeUninit<T>` — Store a value that may not yet be initialized; lets you initialize it safely before treating it as `T`.
- `Discriminant<T>` — Opaque value representing an enum variant’s discriminant.
- `Alignment`† — Represent a valid power-of-two alignment.
- `Assume`† — Configure proof assumptions for experimental transmutation APIs.
- `DropGuard`† — Run a closure when the guard is dropped.
- `MaybeDangling<T>`† — Wrap references or boxes that are allowed to dangle in specific contexts.

### Traits and modules

- `TransmuteFrom<Src>`† — Mark that a source type can be safely transmuted into the implementing type.
- `type_info`† — Experimental module for exposing compile-time type information.

`ManuallyDrop` and `MaybeUninit` have their own associated methods; those are APIs on the types, rather than standalone `std::mem` functions.
