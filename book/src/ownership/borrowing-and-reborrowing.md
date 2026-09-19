# Borrowing and reborrowing

Shared borrows (`&T`) allow observation without transfer. Mutable borrows (`&mut T`) allow temporary exclusive mutation. The borrow checker uses these relationships to prevent a mutation from racing with an incompatible access.

Passing `&mut T` to a helper normally creates a shorter reborrow. Once that helper call is complete, the original mutable borrow can be used again. This is why small helper functions can safely compose mutation without producing two independent mutable owners.

```rust
{{#include ../../../02-ownership-borrowing/examples/004_shared_borrows.rs}}
```

```rust
{{#include ../../../02-ownership-borrowing/examples/005_mutable_borrows_and_reborrowing.rs}}
```

The next Phase 3 tranche will make the borrowing rules and non-lexical lifetimes explicit.

