# std::iter (Trait Iterator) - Quick Reference Methods (capabilities)

- [Trait Iterator API](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

†  = Nightly-only or unstable 

Advancing and inspecting

- `next` — Return the next item, or `None` when exhausted. Required.
- `next_chunk`† — Get the next `N` items as an array, or return the leftovers if there aren’t enough.
- `size_hint` — Give lower and optional upper bounds for the number of remaining items.
- `count` — Consume the iterator and count its items.
- `last` — Consume it and return its last item.
- `advance_by`† — Skip up to `n` items and report whether it ran out.
- `nth` — Consume through and return the item at zero-based index `n`.

Build or adapt an iterator

- `step_by` — Yield every `step`th item.
- `chain` — Yield this iterator’s items, then another iterator’s.
- `zip` — Pair items from two iterators, stopping when either ends.
- `intersperse`† — Put a cloned separator between items.
- `intersperse_with`† — Put a newly generated separator between items.
- `map` — Transform each item.
- `filter` — Keep items that satisfy a predicate.
- `filter_map` — Transform items and discard results that are `None`.
- `enumerate` — Pair each item with its zero-based index.
- `peekable` — Add the ability to inspect the next item without consuming it.
- `skip_while` — Skip items while a predicate is true, then yield the rest.
- `take_while` — Yield items while a predicate is true, then stop.
- `map_while` — Transform items while the closure returns `Some`; stop on `None`.
- `skip` — Skip the first `n` items.
- `take` — Yield at most the first `n` items.
- `scan` — Carry mutable state through items and produce outputs until the closure returns `None`.
- `flat_map` — Map each item to an iterable, then flatten the results.
- `flatten` — Flatten an iterator of iterables by one level.
- `map_windows`† — Apply a closure to each overlapping window of `N` items.
- `fuse` — Ensure the iterator stays exhausted after its first `None`.
- `inspect` — Run a closure on each item without changing what is yielded.
- `by_ref` — Borrow the iterator, allowing it to be used without moving it.

Consume or collect items

- `for_each` — Run a closure on every item.
- `collect` — Collect items into a type implementing `FromIterator`.
- `try_collect` — Collect items that support short-circuiting, returning early on failure.
- `collect_into`† — Extend a supplied collection with the iterator’s items.
- `partition` — Collect items into two collections according to a predicate.
- `partition_in_place`† — Partition a double-ended iterator of mutable references in place; return the split index.
- `is_partitioned`† — Check whether items matching a predicate precede those that don’t.
- `fold` — Reduce items into an accumulator, starting with an initial value.
- `try_fold` — Fold with a closure that can short-circuit on failure.
- `try_for_each` — Run a fallible closure on each item, stopping on failure.
- `reduce` — Combine items using a closure, with no initial value; returns `None` if empty.
- `try_reduce`† — Like `reduce`, but the combining closure can short-circuit on failure.
- `unzip` — Split an iterator of pairs into two collections.

Search and predicates

- `all` — Check whether every item satisfies a predicate.
- `any` — Check whether any item satisfies a predicate.
- `find` — Return the first item matching a predicate.
- `find_map` — Return the first non-`None` result from a mapping closure.
- `try_find`† — Search with a fallible predicate; return early on failure.
- `position` — Return the index of the first item matching a predicate.
- `rposition` — Search from the back and return the matching index.

Ordering and comparison

- `max` / `min` — Return the greatest or least item.
- `max_by` / `min_by` — Find the greatest or least item using a comparator.
- `max_by_key` / `min_by_key` — Find the greatest or least item by a derived key.
- `is_sorted` — Check whether items are in nondecreasing order.
- `is_sorted_by` — Check ordering using a custom comparison predicate.
- `is_sorted_by_key` — Check ordering by a derived key.
- `cmp` — Lexicographically compare items with `Ord`.
- `cmp_by`† — Lexicographically compare using a custom comparator.
- `partial_cmp` — Lexicographically compare items with `PartialOrd`.
- `partial_cmp_by`† — Lexicographically compare using a custom partial comparator.
- `eq` — Check whether two iterators yield equal sequences.
- `eq_by`† — Check sequence equality using a custom comparison closure.
- `ne`, `lt`, `le`, `gt`, `ge` — Compare two sequences for inequality or lexicographic ordering.

Other transformations

- `rev` — Reverse a double-ended iterator.
- `copied` — Copy values out of an iterator of references.
- `cloned` — Clone values out of an iterator of references.
- `cycle` — Repeat an iterator indefinitely.
- `array_chunks`† — Yield items in fixed-size arrays, leaving any incomplete remainder.
- `sum` — Sum the items.
- `product` — Multiply the items together.

