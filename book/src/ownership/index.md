# Ownership and borrowing

Ownership answers who is responsible for a value and its resources. Borrowing temporarily grants access without changing that responsibility. The key model is a relationship, not a bag of syntax rules:

```text
owner ── shared borrow ──> readers may observe
owner ── mutable borrow ─> one temporary writer may update
move  ───────────────────> responsibility transfers to a new owner
```

Start with [ownership and moves](ownership-and-moves.md), then continue to [borrowing and reborrowing](borrowing-and-reborrowing.md).

