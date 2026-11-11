# std::string (Struct String) - Quick Reference Methods (capabilities)

- [Struct String API](https://doc.rust-lang.org/std/string/struct.String.html)

`String` has its own methods and inherits methods from `str` through `Deref`. The list below includes both. † marks methods that are nightly-only or unstable.

### `String` methods

- `new` — Create an empty `String`.
- `with_capacity` — Create an empty `String` with space reserved for at least the requested number of bytes.
- `from_utf8` — Create a `String` from a byte vector, returning an error if the bytes are not valid UTF-8.
- `from_utf8_lossy` — Convert bytes to a string, replacing invalid UTF-8 sequences.
- `from_utf16` — Create a `String` from UTF-16 data, returning an error if it is invalid.
- `from_utf16_lossy` — Convert UTF-16 data to a string, replacing invalid sequences.
- `into_bytes` — Consume the string and return its UTF-8 bytes.
- `into_boxed_str` — Convert the string into a boxed string slice.
- `as_str` — Borrow the string as `&str`.
- `as_mut_str` — Borrow the contents as `&mut str`.
- `as_bytes` — Borrow the UTF-8 bytes.
- `as_mut_vec` — Get mutable access to the underlying byte vector. Unsafe.
- `push` — Append one `char`.
- `push_str` — Append a string slice.
- `pop` — Remove and return the last `char`.
- `insert` — Insert a `char` at a byte index.
- `insert_str` — Insert a string slice at a byte index.
- `remove` — Remove and return the `char` at a byte index.
- `retain` — Keep only characters accepted by a predicate.
- `truncate` — Remove contents after a byte index.
- `clear` — Remove all contents while keeping the allocated capacity.
- `drain` — Remove a range and return an iterator over the removed characters.
- `replace_range` — Replace a range with a string slice.
- `capacity` — Return the allocated capacity in bytes.
- `len` — Return the string’s length in bytes.
- `is_empty` — Check whether the string has no bytes.
- `reserve` — Ensure capacity for at least the requested additional bytes.
- `reserve_exact` — Reserve at least the requested additional bytes, without speculative over-allocation.
- `try_reserve` — Try to reserve additional capacity, returning an error if allocation fails.
- `try_reserve_exact` — Try to reserve the requested additional capacity without speculative over-allocation.
- `shrink_to_fit` — Reduce capacity as much as possible.
- `shrink_to` — Reduce capacity, but not below a requested minimum.
- `leak` — Consume the string and return a `'static` mutable string slice.
- `into_chars`† — Consume the string and return an iterator over its characters.
- `replace_first`† — Replace the first match in place.

### Methods inherited from `str`

**Inspecting and accessing**

- `is_char_boundary` — Check whether a byte index is a UTF-8 character boundary.
- `floor_char_boundary`† — Find the nearest character boundary at or before a byte index.
- `ceil_char_boundary`† — Find the nearest character boundary at or after a byte index.
- `len` — Return the length in bytes.
- `is_empty` — Check whether the string slice is empty.
- `as_bytes` — Borrow the string’s UTF-8 bytes.
- `as_ptr` — Get a pointer to the string’s bytes.
- `as_mut_ptr` — Get a mutable pointer to the bytes. Unsafe.
- `get` — Get a subslice by range, returning `None` if the range is invalid or not on character boundaries.
- `get_mut` — Get a mutable subslice by range, returning `None` if invalid.
- `get_unchecked` — Get a subslice without checking the range. Unsafe.
- `get_unchecked_mut` — Get a mutable subslice without checking the range. Unsafe.
- `split_at` — Split at a byte index, panicking if it is not a character boundary.
- `split_at_mut` — Mutably split at a byte index.
- `split_at_checked` — Split at a byte index, returning `None` if invalid.
- `split_at_mut_checked` — Mutably split at a byte index, returning `None` if invalid.
- `substr_range`† — Find the byte range of a subslice within the string.

**Searching and matching**

- `contains` — Check whether a pattern occurs in the string.
- `starts_with` — Check whether the string begins with a pattern.
- `ends_with` — Check whether the string ends with a pattern.
- `find` — Return the byte index of the first match.
- `rfind` — Return the byte index of the last match.
- `match_indices` — Iterate over matches and their byte indices.
- `rmatch_indices` — Iterate over matches and byte indices from the end.
- `split_once` — Split at the first occurrence of a pattern.
- `rsplit_once` — Split at the last occurrence of a pattern.

**Splitting and iterating**

- `chars` — Iterate over Unicode scalar values (`char`s).
- `char_indices` — Iterate over characters with their byte indices.
- `bytes` — Iterate over UTF-8 bytes.
- `split` — Split around matches of a pattern.
- `split_inclusive` — Split around matches, keeping each match at the end of its preceding part.
- `split_terminator` — Split around matches without yielding a trailing empty part for a final match.
- `rsplit` — Split from the right around matches of a pattern.
- `rsplit_terminator` — Split from the right without a trailing empty part for a final match.
- `split_whitespace` — Split on Unicode whitespace.
- `split_ascii_whitespace` — Split on ASCII whitespace.
- `lines` — Iterate over lines, removing line-ending characters.
- `lines_any`† — Iterate over lines, recognizing additional line-ending characters.
- `split_at_checked` — Split at a byte index if it is a character boundary.
- `split_at_mut_checked` — Mutably split at a byte index if it is a character boundary.

**Trimming and removing prefixes/suffixes**

- `trim` — Remove leading and trailing Unicode whitespace.
- `trim_start` — Remove leading Unicode whitespace.
- `trim_end` — Remove trailing Unicode whitespace.
- `trim_ascii` — Remove leading and trailing ASCII whitespace.
- `trim_ascii_start` — Remove leading ASCII whitespace.
- `trim_ascii_end` — Remove trailing ASCII whitespace.
- `trim_matches` — Remove matching patterns from both ends.
- `trim_start_matches` — Remove matching patterns from the start.
- `trim_end_matches` — Remove matching patterns from the end.
- `strip_prefix` — Remove a matching prefix, if present.
- `strip_suffix` — Remove a matching suffix, if present.
- `strip_prefix_of`† — Return the part of another string after this string, if it is a prefix.
- `strip_suffix_of`† — Return the part of another string before this string, if it is a suffix.

**Replacing**

- `replace` — Replace all matches with another string, returning a new `String`.
- `replacen` — Replace up to a specified number of matches.
- `replace_first`† — Replace the first match in place on a `String`.

**Comparing**

- `eq_ignore_ascii_case` — Compare strings without regard to ASCII letter case.

**Case conversion and escaping**

- `to_lowercase` — Return a lowercase `String` using Unicode rules.
- `to_uppercase` — Return an uppercase `String` using Unicode rules.
- `word_to_titlecase`† — Return a titlecase version of a string assumed to be one word.
- `to_casefold_unnormalized`† — Return a Unicode case-folded string without normalization.
- `escape_debug` — Return an iterator that escapes characters for debug output.
- `escape_default` — Return an iterator that escapes characters using Rust’s default escaping rules.
- `escape_unicode` — Return an iterator that escapes characters as Unicode escape sequences.

**Other**

- `as_str`† — Return the same value as a `&str`; mainly useful for string-like types other than `&str`.
- `into_string`† — Convert a boxed string slice into a `String`.

`String` also supports methods from traits, such as `to_string`, `to_owned`, and comparison methods.
