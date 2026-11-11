# std::result (Module result) - Quick Reference Methods (capabilities)

- [Module result API](https://doc.rust-lang.org/beta/std/result/index.html)


`Result<T, E>` represents either success (`Ok(T)`) or failure (`Err(E)`). This list covers its methods and its associated iterator types. 

† marks nightly-only or unstable methods in the current documentation; 

⚠ marks unsafe methods. 

### Check which variant it is

- `is_ok` — Check whether the result is `Ok`.
- `is_err` — Check whether the result is `Err`.
- `is_ok_and` — Check whether it is `Ok` and its value satisfies a predicate.
- `is_err_and` — Check whether it is `Err` and its error satisfies a predicate.

### Borrow or convert the value

- `as_ref` — Convert `&Result<T, E>` to `Result<&T, &E>`.
- `as_mut` — Convert `&mut Result<T, E>` to `Result<&mut T, &mut E>`.
- `as_deref` — Borrow the `Ok` value through `Deref`, producing a result with a reference to the dereferenced value.
- `as_deref_mut` — Mutably borrow the `Ok` value through `DerefMut`.
- `ok` — Convert to `Option<T>`, discarding any error.
- `err` — Convert to `Option<E>`, discarding any success value.
- `transpose` — Convert `Result<Option<T>, E>` to `Option<Result<T, E>>`.
- `flatten` — Convert `Result<Result<T, E>, E>` into `Result<T, E>`.
- `into_ok` — Consume the result and return its `Ok` value; panic if it is `Err`.
- `into_err` — Consume the result and return its error; panic if it is `Ok`.
- `copied` — Copy the contained value when the result contains a reference.
- `cloned` — Clone the contained value when the result contains a reference.

### Transform or chain results

- `map` — Transform the `Ok` value, leaving `Err` unchanged.
- `map_err` — Transform the error, leaving `Ok` unchanged.
- `map_or` — Transform the `Ok` value, or return a supplied default for `Err`.
- `map_or_else` — Transform the `Ok` value, or compute a fallback from the error.
- `and` — Return another result if this one is `Ok`; otherwise keep this error.
- `and_then` — Call a result-returning function on the `Ok` value; otherwise keep this error.
- `or` — Return this result if it is `Ok`; otherwise return another result.
- `or_else` — Return this result if it is `Ok`; otherwise call a function with its error.
- `inspect` — Call a closure with a reference to the `Ok` value, then return the original result.
- `inspect_err` — Call a closure with a reference to the error, then return the original result.
- `xor` — Return `Ok` only when exactly one of two results is `Ok`; otherwise return an error.

### Extract values

- `expect` — Return the `Ok` value, or panic with a supplied message if it is `Err`.
- `unwrap` — Return the `Ok` value, or panic if it is `Err`.
- `unwrap_or` — Return the `Ok` value, or a supplied default if it is `Err`.
- `unwrap_or_else` — Return the `Ok` value, or compute a fallback from the error.
- `unwrap_or_default` — Return the `Ok` value, or the default value of `T` if it is `Err`.
- `expect_err` — Return the error, or panic with a supplied message if it is `Ok`.
- `unwrap_err` — Return the error, or panic if it is `Ok`.
- `unwrap_unchecked`⚠ — Return the `Ok` value without checking; undefined behavior if it is `Err`.
- `unwrap_err_unchecked`⚠ — Return the error without checking; undefined behavior if it is `Ok`.

### Iterate over the result

- `iter` — Create an iterator over a reference to the `Ok` value; yields no items for `Err`.
- `iter_mut` — Create an iterator over a mutable reference to the `Ok` value; yields no items for `Err`.
- `into_iter` — Consume the result and create an iterator over its `Ok` value; yields no items for `Err`.

### Iterator types in `std::result`

- `Iter<'a, T>` — Iterator over a shared reference to an `Ok` value.
- `IterMut<'a, T>` — Iterator over a mutable reference to an `Ok` value.
- `IntoIter<T>` — Iterator that owns the `Ok` value.

The `?` operator is also commonly used with `Result`: it returns early from a function when the result is `Err`, and otherwise yields the `Ok` value. It is language syntax, not a `Result` method. 
