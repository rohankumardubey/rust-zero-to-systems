# Ownership and moves

Heap-backed values have resources to release. Rust gives each resource one owner; when that owner’s scope ends, cleanup occurs automatically. Assigning a `String` or passing it by value transfers ownership—a move—so the prior binding cannot accidentally free the same allocation again.

Small types such as integers commonly implement `Copy`, so assignment duplicates their bits implicitly. `Clone` is explicit because its work and semantics depend on the type; cloning a `String` allocates an independent buffer.

```rust
{{#include ../../../02-ownership-borrowing/examples/001_stack_and_heap.rs}}
```

```rust
{{#include ../../../02-ownership-borrowing/examples/002_move_semantics.rs}}
```

```rust
{{#include ../../../02-ownership-borrowing/examples/003_copy_and_clone.rs}}
```

Next: [borrowing and reborrowing](borrowing-and-reborrowing.md).

