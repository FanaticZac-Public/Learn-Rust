# std::collections (Struct HashMap) - Quick Reference Methods (capabilities)

- [Module Hashmap API](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

`HashMap<K, V>` is a key-value collection in `std::collections`. It also provides entry and iterator types. † marks nightly-only or unstable APIs; ⚠ marks unsafe APIs. Availability can vary by Rust version and enabled features. 

### Create a map

- `new` — Create an empty map with the default hasher.
- `with_capacity` — Create an empty map with capacity for at least the requested number of entries.
- `with_hasher` — Create an empty map using a specified hasher.
- `with_capacity_and_hasher` — Create a map with a specified capacity and hasher.
- `from` — Create a map from an array of key-value pairs.

### Look up entries

- `get` — Get a reference to the value for a key.
- `get_key_value` — Get references to both the stored key and its value.
- `get_mut` — Get a mutable reference to the value for a key.
- `get_disjoint_mut` — Get mutable references to values for multiple distinct keys; panics if keys overlap.
- `get_disjoint_unchecked_mut`⚠ — Get mutable references for multiple keys without checking that they are distinct.
- `contains_key` — Check whether the map contains a key.
- `entry` — Get an entry API handle for a key, whether or not it is already present.
- `try_insert`† — Insert a key-value pair only if the key is absent; return an error containing the existing value if present.

### Insert, update, and remove

- `insert` — Add a key-value pair, replacing any existing value for that key and returning the old value.
- `remove` — Remove a key and return its value.
- `remove_entry` — Remove a key and return the stored key and value.
- `clear` — Remove all entries while retaining allocated storage.
- `retain` — Keep only entries accepted by a predicate.
- `extract_if` — Remove and yield entries accepted by a predicate.

### Iterate over entries

- `iter` — Iterate over shared references to keys and values.
- `iter_mut` — Iterate over shared key references and mutable value references.
- `keys` — Iterate over shared references to keys.
- `values` — Iterate over shared references to values.
- `values_mut` — Iterate over mutable references to values.
- `into_keys` — Consume the map and iterate over its keys.
- `into_values` — Consume the map and iterate over its values.
- `drain` — Remove all entries and return an iterator over them.

### Capacity and configuration

- `len` — Return the number of entries.
- `is_empty` — Check whether the map contains no entries.
- `capacity` — Return how many entries the map can hold without reallocating.
- `reserve` — Ensure capacity for at least a requested number of additional entries.
- `reserve_exact` — Reserve additional capacity without speculative over-allocation.
- `try_reserve` — Try to reserve additional capacity, returning an error if allocation fails.
- `try_reserve_exact` — Try to reserve exact additional capacity, returning an error if allocation fails.
- `shrink_to_fit` — Reduce capacity as much as possible.
- `shrink_to` — Reduce capacity, but not below a requested minimum.
- `hasher` — Get a reference to the map’s hasher.

### Entry API

- `Entry::key` — Get a reference to the entry’s key.
- `Entry::or_insert` — Insert a supplied value if the key is vacant; return a mutable reference to the value.
- `Entry::or_insert_with` — Lazily create and insert a value if the key is vacant.
- `Entry::or_insert_with_key` — Lazily create a value using a reference to the key if vacant.
- `Entry::and_modify` — Apply a closure to the value if the entry is occupied.
- `Entry::insert_entry` — Insert a value and return an occupied entry handle.
- `Entry::or_default` — Insert the value type’s default if the key is vacant.
- `Entry::insert` — Replace the value in an occupied entry.
- `Entry::get` — Get a shared reference to the occupied entry’s value.
- `Entry::get_mut` — Get a mutable reference to the occupied entry’s value.
- `Entry::into_mut` — Consume the occupied entry handle and return a mutable reference to its value.
- `Entry::remove` — Remove and return the occupied entry’s value.
- `Entry::remove_entry` — Remove and return the occupied key and value.
- `Entry::key` — Get a reference to the occupied entry’s key.
- `Entry::replace_key`† — Replace the key of an occupied entry.
- `Entry::insert` — Insert a value into a vacant entry and return a mutable reference to it.
- `Entry::into_key` — Consume a vacant entry and return its key.

### Iterator and entry types

- `Iter<'a, K, V>` — Iterator over shared references to map entries.
- `IterMut<'a, K, V>` — Iterator over entries with shared key references and mutable value references.
- `Keys<'a, K, V>` — Iterator over shared key references.
- `Values<'a, K, V>` — Iterator over shared value references.
- `ValuesMut<'a, K, V>` — Iterator over mutable value references.
- `IntoIter<K, V>` — Iterator that owns and yields key-value pairs.
- `IntoKeys<K, V>` — Iterator that owns the map and yields keys.
- `IntoValues<K, V>` — Iterator that owns the map and yields values.
- `Drain<'a, K, V>` — Iterator over entries removed from a map by `drain`.
- `ExtractIf<'a, K, V, F>` — Iterator over entries removed by `extract_if`.
- `Entry<'a, K, V>` — Handle to a key that may be vacant or occupied.
- `OccupiedEntry<'a, K, V>` — Handle to an existing entry.
- `VacantEntry<'a, K, V>` — Handle to a key not currently in the map.

The order of entries yielded by a `HashMap`’s iterators is arbitrary. 