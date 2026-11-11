# std::conver (Module convert) - Quick Reference Methods (capabilities)

- [Module convert API](https://doc.rust-lang.org/beta/std/convert/index.html)

`std::convert` provides traits and a function for converting values between types. 

† marks experimental or nightly-only APIs 

### Traits

- `AsRef<T>` — Cheaply borrow a value as `&T`.
  - `as_ref` — Return a reference to the converted value.
- `AsMut<T>` — Cheaply mutably borrow a value as `&mut T`.
  - `as_mut` — Return a mutable reference to the converted value.
- `From<T>` — Convert a value of type `T` into `Self` without failure.
  - `from` — Perform the conversion.
- `Into<T>` — Consume a value and convert it into type `T`.
  - `into` — Perform the conversion.
- `TryFrom<T>` — Try to convert a value of type `T` into `Self`; the conversion may fail.
  - `try_from` — Attempt the conversion, returning a `Result`.
- `TryInto<T>` — Consume a value and attempt to convert it into type `T`.
  - `try_into` — Attempt the conversion, returning a `Result`.
- `BoundedCastFromIntExperimental`† — Convert integers to a target type using wrapping or saturating behavior.
- `CheckedCastFromIntExperimental`† — Attempt a checked integer conversion that can fail.
- `FloatToIntExperimental`† — Supporting trait for experimental float-to-integer conversion APIs.

### Function

- `identity` — Return its argument unchanged.

### Enum

- `Infallible` — An uninhabited error type for conversions that cannot fail.

Implement `From` rather than `Into`, and `TryFrom` rather than `TryInto`: Rust provides the corresponding `Into` or `TryInto` implementation automatically. 
