# Single-Producer-Multiple-Consumer Channel for Tokio

The [tokio](https://tokio.rs/) async runtime provides
[several kinds of channels](https://tokio.rs/tokio/tutorial/channels)
for [communicating sequential processes](https://en.wikipedia.org/wiki/Communicating_sequential_processes).

There is no channel for a single producer and multiple consumers
that supports sending each message to only one consumer.
`@DocWilco` on the Rust `#beginners` Discord channel
suggested combining `mpsc` with `oneshot` channels.
I recognized this pattern from [Golang](https://go.dev/)
and found it to work,
but it is confusing to look at.
So I created an abstraction to wrap all the levels of send and receive.

This demo only uses a `usize`-type message, but it should work
for any message type that fits the required traits.

    Send
    Sync
    std::fmt::Debug

... and lifetime, `'static`.

I'd like to see pull requests from people who have ideas about making it more flexible.
