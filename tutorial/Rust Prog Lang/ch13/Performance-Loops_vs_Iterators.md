


# Performance in Loops vs. Iterators

To determine whether to use loops or iterators, you need to know which implementation is faster: the version of the search function with an explicit for loop or the version with iterators.

We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a String and looking for the word the in the contents. 

Here are the results of the benchmark on the version of search using the for loop and the version using iterators:

```console
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

The two implementations have similar performance! We won’t explain the benchmark code here because the point is not to prove that the two versions are equivalent but to get a general sense of how these two implementations compare performance-wise.

For a more comprehensive benchmark, you should check using various texts of various sizes as the contents, different words and words of different lengths as the query, and all kinds of other variations. 

The point is this: Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. 

Iterators are one of Rust’s zero-cost abstractions, by which we mean that using the abstraction imposes no additional runtime overhead.

<table cellpadding="0" cellspacing="0"  bgcolor="#090909"   border="1"
  frame="box"   rules="none">
  <tr>
    <td width="66" valign="middle">
      <img
        src="../../../images/ed-images/ed_hack_1.png"
        width="40"
        height="40"
        alt="Author Git Logo"
        align="center"
      >
    </td>
    <td>
 <strong>
Now that you mention it: 
</strong>
    </td>
  </tr>
</table>

[Zero Cost Abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html)

In many cases, Rust code using iterators compiles to the same assembly you’d write by hand. Optimizations such as loop unrolling and eliminating bounds checking on array access apply and make the resultant code extremely efficient. Now that you know this, you can use iterators and closures without fear! They make code seem like it’s higher level but don’t impose a runtime performance penalty for doing so.

## Summary
Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.