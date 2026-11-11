# std::cmp (Module cmp) - Quick Reference Methods (capabilities)

- [Module cmp API](https://doc.rust-lang.org/beta/std/cmp/index.html)

`std::cmp` provides comparison traits, ordering types, and functions for selecting minimum and maximum values. 

† marks experimental or nightly-only APIs in the current documentation. 

### Functions

- `max` — Return the greater of two values.
- `min` — Return the lesser of two values.
- `max_by` — Return the greater value using a comparison closure.
- `min_by` — Return the lesser value using a comparison closure.
- `max_by_key` — Return the value with the greater derived key.
- `min_by_key` — Return the value with the lesser derived key.
- `largest`† — Return the greatest value from an iterator of values.
- `smallest`† — Return the least value from an iterator of values.
- `minmax`† — Return two values in minimum-then-maximum order.
- `minmax_by`† — Return the minimum and maximum using a comparison closure.
- `minmax_by_key`† — Return the minimum and maximum using a key-producing closure.

### Traits

- `PartialEq<Rhs>` — Enable `==` and `!=` comparisons, which may not be defined for every pair of values.
  - `eq` — Test whether values are equal.
  - `ne` — Test whether values are unequal.
- `Eq` — Mark a type whose equality comparison is an equivalence relation. This trait has no methods.
- `PartialOrd<Rhs>` — Enable `<`, `<=`, `>`, and `>=` comparisons when values may be incomparable.
  - `partial_cmp` — Return the relative ordering, or `None` if values are incomparable.
  - `lt` — Test whether one value is less than another.
  - `le` — Test whether one value is less than or equal to another.
  - `gt` — Test whether one value is greater than another.
  - `ge` — Test whether one value is greater than or equal to another.
- `Ord` — Mark a type with a total ordering.
  - `cmp` — Compare two values, returning `Less`, `Equal`, or `Greater`.
  - `max` — Return the greater of two values.
  - `min` — Return the lesser of two values.
  - `clamp` — Restrict a value to an inclusive lower and upper bound.

### Types

- `Ordering` — Enum representing a comparison result: `Less`, `Equal`, or `Greater`.
  - `reverse` — Swap `Less` and `Greater`; leave `Equal` unchanged.
  - `then` — Use another ordering if this one is `Equal`.
  - `then_with` — Compute and use another ordering if this one is `Equal`.
  - `is_lt` — Check whether the ordering is `Less`.
  - `is_eq` — Check whether the ordering is `Equal`.
  - `is_gt` — Check whether the ordering is `Greater`.
- `Reverse<T>` — Wrapper that reverses a value’s ordering, useful when sorting in descending order.

### Derive macros

- `PartialEq` — Generate a `PartialEq` implementation.
- `Eq` — Generate an `Eq` implementation.
- `PartialOrd` — Generate a `PartialOrd` implementation.
- `Ord` — Generate an `Ord` implementation.

The derive macros work on structs and enums; the generated comparison follows their fields and declaration order.
