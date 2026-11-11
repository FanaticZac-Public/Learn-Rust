# std::ops (Module ops) - Quick Reference Methods (capabilities)

- [Module ops API](https://doc.rust-lang.org/beta/std/ops/index.html)

`std::ops` contains traits that power Rust operators, range types, and a few related types. “Methods” here means the module’s public APIs; many operator traits define a required method that you implement to customize an operator. Exact contents may vary by Rust version.

### Operator traits

- `Add` — Implements `+` with `add`.
- `Sub` — Implements `-` with `sub`.
- `Mul` — Implements `*` with `mul`.
- `Div` — Implements `/` with `div`.
- `Rem` — Implements `%` with `rem`.
- `Neg` — Implements unary negation (`-`) with `neg`.
- `Not` — Implements logical or bitwise negation (`!`) with `not`.
- `BitAnd` — Implements bitwise AND (`&`) with `bitand`.
- `BitOr` — Implements bitwise OR (`|`) with `bitor`.
- `BitXor` — Implements bitwise XOR (`^`) with `bitxor`.
- `Shl` — Implements left shift (`<<`) with `shl`.
- `Shr` — Implements right shift (`>>`) with `shr`.
- `AddAssign` — Implements `+=` with `add_assign`.
- `SubAssign` — Implements `-=` with `sub_assign`.
- `MulAssign` — Implements `*=` with `mul_assign`.
- `DivAssign` — Implements `/=` with `div_assign`.
- `RemAssign` — Implements `%=` with `rem_assign`.
- `BitAndAssign` — Implements `&=` with `bitand_assign`.
- `BitOrAssign` — Implements `|=` with `bitor_assign`.
- `BitXorAssign` — Implements `^=` with `bitxor_assign`.
- `ShlAssign` — Implements `<<=` with `shl_assign`.
- `ShrAssign` — Implements `>>=` with `shr_assign`.

These traits generally have a required operator method and an associated `Output` type. Their generic `Rhs` type lets the right-hand operand differ from the implementing type.

### Dereferencing, indexing, and calling

- `Deref` — Enables dereference (`*value`) and automatic deref coercion with `deref`.
- `DerefMut` — Enables mutable dereference with `deref_mut`.
- `Index` — Enables indexing (`container[index]`) with `index`.
- `IndexMut` — Enables mutable indexing with `index_mut`.
- `Fn` — Enables calling a value like a function when it only needs shared access.
- `FnMut` — Enables calling a value like a function when it may mutate its state.
- `FnOnce` — Enables calling a value like a function when the call may consume it.
- `Drop` — Runs cleanup code when a value is dropped, using `drop`.

The `Fn`, `FnMut`, and `FnOnce` traits are mainly implemented by closures and function-like values; their call methods use special Rust syntax.

### Range types

- `Range<Idx>` — Represents a half-open range, `start..end`.
- `RangeFrom<Idx>` — Represents a range from a start onward, `start..`.
- `RangeFull` — Represents an unbounded range, `..`.
- `RangeInclusive<Idx>` — Represents a range including both endpoints, `start..=end`.
- `RangeTo<Idx>` — Represents a range up to, but not including, an end, `..end`.
- `RangeToInclusive<Idx>` — Represents a range up to and including an end, `..=end`.
- `RangeBounds<T>` — A trait for types that describe a range’s start and end bounds.
- `Bound<T>` — Represents an included, excluded, or unbounded endpoint.

### Other types

- `ControlFlow<B, C>` — Represents either `Continue(C)` or `Break(B)`, commonly used for early exit while processing items.
- `ControlFlow::is_break` — Check whether the value is `Break`.
- `ControlFlow::is_continue` — Check whether the value is `Continue`.
- `ControlFlow::break_value` — Get the break value as an `Option`.
- `ControlFlow::continue_value` — Get the continue value as an `Option`.
- `ControlFlow::map_break` — Transform the value in the `Break` case.
- `ControlFlow::map_continue` — Transform the value in the `Continue` case.
