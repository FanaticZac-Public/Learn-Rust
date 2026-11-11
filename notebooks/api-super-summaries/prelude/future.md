# std::future (Module future) - Quick Reference Methods (capabilities)

- [Module future API](https://doc.rust-lang.org/beta/std/future/index.html)

`std::future` provides Rust’s core future traits, a few simple future types and constructors, and a macro for joining futures. The standard library’s `Future` trait has only one method; convenience methods such as `map` and `then` are provided by external extension traits, not `std::future`.

† marks nightly-only or experimental APIs.

### Traits

- `Future` — Represents an asynchronous computation that may complete later.
  - `poll` — Check whether the future is ready; return `Poll::Ready(value)` when complete or `Poll::Pending` when it needs more time. Required.
- `IntoFuture` — Converts a value into a future, used by `.await`.
  - `into_future` — Convert `self` into a future.

### Functions

- `ready` — Create a future that is immediately ready with a value.
- `pending` — Create a future that never completes.
- `poll_fn` — Create a future whose polling behavior is defined by a closure.
- `async_drop_in_place`†⚠ — Asynchronously drop a value in place.

### Types

- `Ready<T>` — A future that immediately returns a stored value.
- `Pending<T>` — A future that never resolves to a value of type `T`.
- `PollFn<F>` — A future that uses a closure to determine what `poll` returns.
- `AsyncDrop`† — Trait for types that support asynchronous destruction.

### Macro

- `join!`† — Poll multiple futures concurrently and return a tuple of their results once all are complete.

`async` blocks and `async` functions also create futures, but they are language features, not methods in `std::future`.
