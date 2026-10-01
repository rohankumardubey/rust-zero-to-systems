# Pin and Unpin

Most Rust values may move freely. A pinned pointer adds a guarantee that a `!Unpin` value will not
be moved out through that pointer after pinning. `Unpin` is an auto trait: ordinary types such as
`String` implement it, so pinning does not restrict their movement. `PhantomPinned` opts a type out
of `Unpin`, but creating a self-referential value still requires a carefully designed abstraction;
the marker alone does not make internal pointers safe.

`Pin<Box<T>>` also keeps the allocation alive while the box owns it. Pinning itself is not an
ownership mechanism, and dropping the owner still drops the value. Async functions commonly produce
state machines that may be `!Unpin`; executors poll them through pinned references.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/014_pin_and_unpin.rs}}
```

This example demonstrates the safe API boundary without constructing a self-reference. Pin
projection and the `Future::poll` contract are revisited with async Rust.
