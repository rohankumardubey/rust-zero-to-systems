# Partial moves and drop scopes

Destructuring can move a non-`Copy` field while copying a `Copy` field. After a partial move, Rust prevents use of the original aggregate in ways that would require the moved field. This is a precise accounting rule, not a limitation of destructuring.

Scope provides deterministic resource cleanup (RAII): inner values are dropped when their scope ends, while values owned by an outer scope remain available. Later `Drop` lessons make custom cleanup behavior explicit.

```rust
{{#include ../../../02-ownership-borrowing/examples/006_partial_moves_and_drop_scopes.rs}}
```

