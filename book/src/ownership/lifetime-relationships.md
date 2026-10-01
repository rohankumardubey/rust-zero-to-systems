# Lifetime relationships

Lifetimes describe how long references remain valid relative to one another. They are not timers and do not extend a value’s lifetime; annotations express a relationship the compiler can verify. The most common case is a function returning one of its input borrows, which needs one named relationship between all relevant references.

Structs that store references carry the same relationship in their type. Elision rules infer common function and method cases, while `'static` is appropriate only for data valid for the full program, such as a string literal.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/001_lifetime_annotations.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/002_struct_lifetimes.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/003_elision_and_static.rs}}
```

Multiple lifetime parameters keep separate borrows separate, while an outlives bound states a
specific validity relationship. Shared references are covariant in their lifetime, so a longer
borrow can be shortened safely. Higher-ranked trait bounds use `for<'a>` when a callback must work
for each borrow lifetime chosen by its caller. These rules describe relationships; none of them
extends the lifetime of the underlying data.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/011_multiple_lifetimes_and_bounds.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/012_lifetime_variance.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/013_higher_ranked_trait_bounds.rs}}
```

For the related pinning contract, see [Pin and Unpin](pin-and-unpin.md). Pinning is an address
stability guarantee used by some abstractions; it does not keep the allocation alive independently
of its owner.
