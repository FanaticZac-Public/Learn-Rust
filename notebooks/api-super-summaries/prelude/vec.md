# std::vec (Struct Vec ) - Quick Reference Methods (capabilities)

- [Struct Vec API](https://doc.rust-lang.org/std/vec/struct.Vec.html)

`Vec<T>` has its own methods and inherits slice methods through `Deref<Target = [T]>`. This list covers those methods; trait-provided methods such as `clone` and `into_iter` are not included. 

† marks nightly-only or unstable methods

### `Vec<T>` methods

**Create and convert**

- `new` — Create an empty vector.
- `with_capacity` — Create an empty vector with at least the requested capacity.
- `from` — Create a vector from a compatible value, such as an array or slice.
- `from_elem` — Create a vector containing a repeated value.
- `from_iter` — Create a vector from an iterator.
- `into_boxed_slice` — Convert the vector into a boxed slice.
- `into_flattened` — Flatten a vector of arrays into a vector of their elements.
- `into_parts`† — Consume the vector and return its raw parts.
- `from_parts`† — Construct a vector from raw parts.
- `from_raw_parts` — Construct a vector from a raw pointer, length, and capacity. Unsafe.
- `from_raw_parts_in`† — Construct a vector from raw parts using a specified allocator. Unsafe.
- `leak`† — Consume the vector and leak its contents, returning a mutable slice.
- `into_raw_parts`† — Consume the vector and return its pointer, length, and capacity.
- `into_raw_parts_with_alloc`† — Consume the vector and return its raw parts, including its allocator.

**Access and capacity**

- `as_slice` — Borrow the vector as a slice.
- `as_mut_slice` — Borrow the vector as a mutable slice.
- `as_ptr` — Get a pointer to the vector’s elements.
- `as_mut_ptr` — Get a mutable pointer to the vector’s elements.
- `len` — Return the number of elements.
- `is_empty` — Check whether the vector contains no elements.
- `capacity` — Return how many elements the vector can hold without reallocating.
- `spare_capacity_mut` — Get the uninitialized spare capacity as a mutable slice of `MaybeUninit<T>`.
- `set_len` — Set the vector’s length. Unsafe.
- `allocator`† — Get a reference to the vector’s allocator.

**Add, remove, and resize**

- `push` — Add an element to the end.
- `pop` — Remove and return the last element.
- `insert` — Insert an element at an index, shifting later elements.
- `remove` — Remove and return the element at an index, shifting later elements.
- `swap_remove` — Remove and return an element by swapping in the last element.
- `append` — Move all elements from another vector onto the end.
- `extend_from_slice` — Append cloned elements from a slice.
- `extend_from_within` — Append cloned elements from a range of the vector.
- `resize` — Change the length, filling new slots with a cloned value.
- `resize_with` — Change the length, generating new elements with a closure.
- `truncate` — Remove elements after a specified length.
- `clear` — Remove all elements while keeping the allocated capacity.
- `retain` — Keep only elements accepted by a predicate.
- `retain_mut` — Mutably inspect elements and keep those accepted by a predicate.
- `dedup` — Remove consecutive duplicate elements.
- `dedup_by` — Remove consecutive elements according to a comparison closure.
- `dedup_by_key` — Remove consecutive elements with matching derived keys.
- `extract_if` — Remove and yield elements in a range when a predicate returns true.
- `drain` — Remove a range and return an iterator over the removed elements.
- `splice` — Replace a range with elements from an iterator, yielding the removed elements.

**Reserve and shrink capacity**

- `reserve` — Ensure capacity for at least a requested number of additional elements.
- `reserve_exact` — Reserve at least the requested additional capacity without speculative over-allocation.
- `try_reserve` — Try to reserve additional capacity, returning an error if allocation fails.
- `try_reserve_exact` — Try to reserve the requested additional capacity without speculative over-allocation.
- `shrink_to_fit` — Reduce capacity as much as possible.
- `shrink_to` — Reduce capacity, but not below a requested minimum.

### Methods inherited from `[T]`

**Inspect and access elements**

- `first` — Get the first element, if present.
- `first_mut` — Mutably get the first element, if present.
- `last` — Get the last element, if present.
- `last_mut` — Mutably get the last element, if present.
- `get` — Get an element or subslice by index or range, returning `None` if out of bounds.
- `get_mut` — Mutably get an element or subslice, returning `None` if out of bounds.
- `get_unchecked` — Get an element or subslice without bounds checks. Unsafe.
- `get_unchecked_mut` — Mutably get an element or subslice without bounds checks. Unsafe.
- `split_at` — Split into two slices at an index.
- `split_at_mut` — Mutably split into two slices at an index.
- `split_at_checked` — Split at an index, returning `None` if it is out of bounds.
- `split_at_mut_checked` — Mutably split at an index, returning `None` if it is out of bounds.
- `as_ptr_range` — Get the address range covered by the slice’s elements.
- `each_ref` — Make an array of references to the elements.
- `each_mut` — Make an array of mutable references to the elements.
- `split_first` — Return the first element and the remaining slice, if nonempty.
- `split_first_mut` — Mutably return the first element and the remaining slice.
- `split_last` — Return the last element and the preceding slice, if nonempty.
- `split_last_mut` — Mutably return the last element and the preceding slice.
- `split_first_chunk` — Split off the first fixed-size array, if it fits.
- `split_first_chunk_mut` — Mutably split off the first fixed-size array.
- `split_last_chunk` — Split off the last fixed-size array, if it fits.
- `split_last_chunk_mut` — Mutably split off the last fixed-size array.
- `first_chunk` — Get the first fixed-size array, if it fits.
- `first_chunk_mut` — Mutably get the first fixed-size array.
- `last_chunk` — Get the last fixed-size array, if it fits.
- `last_chunk_mut` — Mutably get the last fixed-size array.
- `as_chunks` — Split a slice into fixed-size array chunks and a remainder.
- `as_chunks_mut` — Mutably split a slice into fixed-size array chunks and a remainder.
- `as_rchunks` — Split a slice into fixed-size array chunks from the end and a remainder.
- `as_rchunks_mut` — Mutably split into fixed-size array chunks from the end.
- `array_chunks`† — Iterate over fixed-size array chunks.
- `array_chunks_mut`† — Mutably iterate over fixed-size array chunks.
- `rchunks` — Iterate over chunks from the end.
- `rchunks_mut` — Mutably iterate over chunks from the end.
- `chunks` — Iterate over non-overlapping chunks.
- `chunks_mut` — Mutably iterate over non-overlapping chunks.
- `chunks_exact` — Iterate over fixed-size chunks, leaving a remainder.
- `chunks_exact_mut` — Mutably iterate over fixed-size chunks, leaving a remainder.
- `rchunks_exact` — Iterate over fixed-size chunks from the end, leaving a remainder.
- `rchunks_exact_mut` — Mutably iterate over fixed-size chunks from the end.
- `windows` — Iterate over overlapping windows of a specified length.
- `chunk_by` — Group consecutive elements into slices using a predicate.
- `chunk_by_mut` — Mutably group consecutive elements into slices using a predicate.
- `split` — Iterate over subslices separated by a predicate.
- `split_mut` — Mutably iterate over subslices separated by a predicate.
- `split_inclusive` — Split into subslices, keeping each matching element at the end of its subslice.
- `split_inclusive_mut` — Mutably split while keeping each matching element at the end.
- `splitn` — Split into at most a specified number of subslices.
- `splitn_mut` — Mutably split into at most a specified number of subslices.
- `rsplit` — Iterate over subslices separated by a predicate, from the end.
- `rsplit_mut` — Mutably split from the end.
- `rsplitn` — Split into at most a specified number of subslices, from the end.
- `rsplitn_mut` — Mutably split into at most a specified number of subslices, from the end.

**Search and predicates**

- `contains` — Check whether the slice contains a value.
- `starts_with` — Check whether the slice begins with another slice.
- `ends_with` — Check whether the slice ends with another slice.
- `strip_prefix` — Return the slice without a matching prefix, if present.
- `strip_suffix` — Return the slice without a matching suffix, if present.
- `strip_prefix_of`† — Find this slice as the prefix of another slice.
- `strip_suffix_of`† — Find this slice as the suffix of another slice.
- `iter` — Iterate over shared references to the elements.
- `iter_mut` — Iterate over mutable references to the elements.
- `iter_chunked`† — Iterate over fixed-size array chunks.
- `iter_chunked_mut`† — Mutably iterate over fixed-size array chunks.
- `position` — Return the index of the first element matching a predicate.
- `partition_point` — Find the boundary between elements matching a predicate and those that do not.
- `binary_search` — Binary-search a sorted slice for a value.
- `binary_search_by` — Binary-search using a comparison closure.
- `binary_search_by_key` — Binary-search by a derived key.
- `is_sorted` — Check whether the elements are sorted.
- `is_sorted_by` — Check whether the elements are sorted using a comparator.
- `is_sorted_by_key` — Check whether the elements are sorted by a derived key.

**Reorder and transform**

- `reverse` — Reverse the elements in place.
- `rotate_left` — Rotate elements left by a specified amount.
- `rotate_right` — Rotate elements right by a specified amount.
- `swap` — Exchange two elements by index.
- `swap_with_slice` — Exchange all elements with another same-length slice.
- `sort` — Sort elements in place using `Ord`.
- `sort_unstable` — Sort in place without preserving the order of equal elements.
- `sort_by` — Sort using a comparator.
- `sort_unstable_by` — Sort without stability using a comparator.
- `sort_by_key` — Sort by a derived key.
- `sort_unstable_by_key` — Sort without stability by a derived key.
- `sort_by_cached_key` — Sort by a derived key, caching each key.
- `select_nth_unstable` — Reorder so the element at an index is in its sorted position; partition around it.
- `select_nth_unstable_by` — Do the same using a comparator.
- `select_nth_unstable_by_key` — Do the same using a derived key.
- `fill` — Replace every element with a cloned value.
- `fill_with` — Replace every element with a value generated by a closure.
- `clone_from_slice` — Clone values from another same-length slice into this slice.
- `copy_from_slice` — Copy values from another same-length slice into this slice.
- `copy_within` — Copy a range of elements to another position within the slice.
- `to_vec` — Clone the slice into a new vector.
- `repeat` — Create a new vector by repeating the slice.
- `to_vec_in`† — Clone the slice into a vector using a specified allocator.

**Compare**

- `eq` — Check whether two slices contain equal elements in the same order.
- `ne` — Check whether two slices differ.
- `cmp` — Lexicographically compare slices using `Ord`.
- `partial_cmp` — Lexicographically compare slices using `PartialOrd`.

Indexing with `v[index]` and iteration with `for item in &v` are also supported through traits, rather than being `Vec` methods. The exact available methods vary with Rust version and enabled nightly features. <citation src="1"></citation>
