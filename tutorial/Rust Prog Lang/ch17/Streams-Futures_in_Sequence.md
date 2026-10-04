# Streams: Futures in Sequence

## Super Summary

```rust
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

## Streams: Futures in Sequence

- Recall how we used the receiver for our async channel earlier in this chapter in the “Message Passing” section. 
- The async recv method produces a sequence of items over time. 
- This is an instance of a much more general pattern known as a stream. 
- Many concepts are naturally represented as streams: items becoming available in a queue, chunks of data being pulled incrementally from the filesystem when the full data set is too large for the computer’s memory, or data arriving over the network over time. 
- Because streams are futures, we can use them with any other kind of future and combine them in interesting ways. 
- For example, we can batch up events to avoid triggering too many network calls, set timeouts on sequences of long-running operations, or throttle user interface events to avoid doing needless work.

- We saw a sequence of items back in Chapter 13, when we looked at the Iterator trait in “The Iterator Trait and the next Method” section, but there are two differences between iterators and the async channel receiver. 
- The first difference is time: iterators are synchronous, while the channel receiver is asynchronous. 
- The second difference is the API. When working directly with Iterator, we call its synchronous next method. With the trpl::Receiver stream in particular, we called an asynchronous recv method instead. 
- Otherwise, these APIs feel very similar, and that similarity isn’t a coincidence. 
- A stream is like an asynchronous form of iteration. Whereas the trpl::Receiver specifically waits to receive messages, though, the general-purpose stream API is much broader: it provides the next item the way Iterator does, but asynchronously.

- The similarity between iterators and streams in Rust means we can actually create a stream from any iterator. 
- As with an iterator, we can work with a stream by calling its next method and then awaiting the output.

```rust
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

- We start with an array of numbers, which we convert to an iterator and then call map on to double all the values. 
- Then we convert the iterator into a stream using the trpl::stream_from_iter function. 
- Next, we loop over the items in the stream as they arrive with the while let loop.
- We need the right trait in scope to be able to use the next method. Given our discussion so far, you might reasonably expect that trait to be Stream, but it’s actually StreamExt. Short for extension, Ext is a common pattern in the Rust community for extending one trait with another.
- The Stream trait defines a low-level interface that effectively combines the Iterator and Future traits. 
- StreamExt supplies a higher-level set of APIs on top of Stream, including the next method as well as other utility methods similar to those provided by the Iterator trait. 
- Stream and StreamExt are not yet part of Rust’s standard library, but most ecosystem crates use similar definitions.