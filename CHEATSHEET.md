# Rust cheatsheet

This file will become a high-density revision index as lessons land. Canonical explanations and runnable examples live in the subject directories and book.

## Commands

```bash
cargo run -p core-rust-lessons --example 001_hello_world
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps --open
mdbook serve book --open
```

## Navigation shortcuts

- Ownership/moves/borrows: Phase 3, `02-ownership-borrowing/`
- Traits/generics/dispatch: Phase 4, `03-types-generics-traits/`
- Lifetimes/smart pointers: Phase 3, `07-lifetimes-smart-pointers/`
- Threads/channels/atomics: Phase 6, `09-concurrency/`
- Futures/Tokio: Phase 7, `10-async-rust/`
- Unsafe/layout/provenance: Phase 8, `12-unsafe-rust/`

Only implemented concepts should acquire syntax summaries here.

