# Ownership and borrowing

Ownership answers who is responsible for a value and its resources. Borrowing temporarily grants access without changing that responsibility. The key model is a relationship, not a bag of syntax rules:

```text
owner ── shared borrow ──> readers may observe
owner ── mutable borrow ─> one temporary writer may update
move  ───────────────────> responsibility transfers to a new owner
```

Start with [ownership and moves](ownership-and-moves.md), then continue to [borrowing and reborrowing](borrowing-and-reborrowing.md).

For the later topics, see [lifetime relationships](lifetime-relationships.md), [smart pointers and interior mutability](smart-pointers-and-interior-mutability.md), [RAII, copy-on-write, and non-lexical lifetimes](raii-cow-and-nll.md), and [Pin and Unpin](pin-and-unpin.md).
