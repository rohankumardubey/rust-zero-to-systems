# RAII, copy-on-write, and non-lexical lifetimes

RAII means a resource is acquired by an owning value and released when that owner drops. Implementing `Drop` is appropriate for local cleanup; when closing can fail and the caller needs to know, expose an explicit close operation too.

`Cell<T>` provides simple single-threaded interior mutation for `Copy` values. `Cow<'a, T>` represents either a borrowed value or an owned one, avoiding an allocation when no transformation is necessary. Non-lexical lifetimes let the compiler end a borrow at its last use, so later compatible access can proceed in the same block.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/007_raii_and_drop.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/008_cell.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/009_cow.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/010_non_lexical_lifetimes.rs}}
```

