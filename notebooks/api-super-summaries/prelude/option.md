# std::option (Module option) - Quick Reference Methods (capabilities)

- [Module option API](https://doc.rust-lang.org/beta/std/option/index.html)

`Option<T>` represents either `Some(T)` or `None`. This list covers its methods and iterator types.

† marks nightly-only or unstable APIs in the current documentation;

⚠ marks unsafe methods.

### Check the value

- `is_some` — Check whether the option is `Some`.
- `is_none` — Check whether the option is `None`.
- `is_some_and` — Check whether it is `Some` and its value satisfies a predicate.
- `is_none_or` — Check whether it is `None`, or its value satisfies a predicate.

### Borrow or convert the value

- `as_ref` — Convert `&Option<T>` to `Option<&T>`.
- `as_mut` — Convert `&mut Option<T>` to `Option<&mut T>`.
- `as_deref` — Borrow the contained value through `Deref`.
- `as_deref_mut` — Mutably borrow the contained value through `DerefMut`.
- `as_pin_ref` — Convert a pinned shared reference to `Option<Pin<&T>>`.
- `as_pin_mut` — Convert a pinned mutable reference to `Option<Pin<&mut T>>`.
- `as_slice` — Return a slice containing the value, or an empty slice for `None`.
- `as_mut_slice` — Return a mutable slice containing the value, or an empty slice for `None`.
- `iter` — Create an iterator over a shared reference to the value; empty for `None`.
- `iter_mut` — Create an iterator over a mutable reference to the value; empty for `None`.
- `into_iter` — Consume the option and create an iterator over its value; empty for `None`.
- `into_flat_iter`† — Consume the option and return an iterator that can flatten it with other iterables.
- `copied` — Copy the contained value when the option contains a reference.
- `cloned` — Clone the contained value when the option contains a reference.
- `transpose` — Convert `Option<Result<T, E>>` to `Result<Option<T>, E>`.
- `flatten` — Convert `Option<Option<T>>` to `Option<T>`.

### Transform or combine options

- `map` — Transform the value if it is `Some`.
- `map_or` — Transform the value, or return a supplied default for `None`.
- `map_or_else` — Transform the value, or compute a fallback for `None`.
- `map_or_default`† — Transform the value, or return the output type’s default for `None`.
- `and` — Return another option if this one is `Some`; otherwise return `None`.
- `and_then` — Call an option-returning function on the value; return `None` if this option is `None`.
- `or` — Return this option if it is `Some`; otherwise return another option.
- `or_else` — Return this option if it is `Some`; otherwise compute another option.
- `xor` — Return `Some` only if exactly one of this option and another option is `Some`.
- `filter` — Keep the value as `Some` only if it satisfies a predicate.
- `inspect` — Call a closure with a reference to the value, then return the original option.
- `zip` — Combine two `Some` values into a pair; return `None` if either option is `None`.
- `zip_with`† — Combine two contained values with a closure; return `None` if either option is `None`.

### Get a value or fallback

- `unwrap` — Return the value, or panic if the option is `None`.
- `expect` — Return the value, or panic with a supplied message if the option is `None`.
- `unwrap_unchecked`⚠ — Return the value without checking; undefined behavior if the option is `None`.
- `unwrap_or` — Return the value, or a supplied default for `None`.
- `unwrap_or_else` — Return the value, or compute a fallback for `None`.
- `unwrap_or_default` — Return the value, or the type’s default value for `None`.

### Modify an option in place

- `insert` — Store a value, replacing and dropping any existing value; return a mutable reference to the stored value.
- `get_or_insert` — Keep an existing value or insert a supplied one; return a mutable reference to it.
- `get_or_insert_default` — Keep an existing value or insert the type’s default value.
- `get_or_insert_with` — Keep an existing value or lazily compute and insert one.
- `get_or_try_insert_with`† — Keep an existing value or try to compute and insert one, returning early if computation fails.
- `take` — Remove and return the value, leaving `None`.
- `take_if` — Remove and return the value if a predicate on a mutable reference returns true.
- `replace` — Replace the option’s value with `Some(new_value)` and return its previous value.

### Iterator types in `std::option`

- `IntoIter<T>` — Iterator that owns the value in `Some`; yields no items for `None`.
- `Iter<'a, T>` — Iterator over a shared reference to the value in `Some`.
- `IterMut<'a, T>` — Iterator over a mutable reference to the value in `Some`.
- `OptionFlattenExperimental`† — Iterator type returned by `into_flat_iter`.

`?` can also be used with `Option`: it returns early with `None` when the value is absent, and otherwise yields the contained value. It is language syntax, not an `Option` method.
