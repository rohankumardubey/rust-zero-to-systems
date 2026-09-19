# References and iteration

References borrow rather than own. An immutable reference, `&T`, permits reading; a mutable reference, `&mut T`, permits updates through a controlled access path. The detailed exclusivity and lifetime rules are the subject of Phase 3, but the basic API shape is worth learning here: take `&T` to observe caller-owned data and `&mut T` to update it.

`loop` produces a value through `break value`; `while` repeats as long as its condition holds; ranges such as `1..=3` express ordered bounds; and `if let` handles one pattern without writing a full `match`.

```rust
{{#include ../../../01-core-rust/examples/016_references_and_dereference.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/017_loops_ranges_and_if_let.rs}}
```

