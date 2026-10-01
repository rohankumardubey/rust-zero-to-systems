# Ownership and memory baseline

Primary baselines: [The Rust Programming Language — Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html), [Rust Reference — subtyping and variance](https://doc.rust-lang.org/reference/subtyping.html), [standard library `Pin`](https://doc.rust-lang.org/std/pin/struct.Pin.html), and the Rustonomicon for unsafe-adjacent models.

Audit: stack/heap; ownership/moves; `Copy`/`Clone`; shared/mutable borrowing; reborrowing; partial moves; drop scopes; dangling prevention; slices/NLL/RAII/`Drop`; `Box`/`Rc`/`Arc`/`Weak`; `Cell`/`RefCell`/`UnsafeCell`; `Cow`; `Pin`/`Unpin`; lifetimes, elision, bounds, `'static`, subtyping, variance, HRTBs.

Status: Phase 3 is complete. Stack/heap intuition, moves, `Copy`/`Clone`, shared and mutable borrows, reborrowing, partial moves, drop scopes, RAII/`Drop`, NLL, multiple lifetime relationships and bounds, lifetime variance, higher-ranked trait bounds, `Pin`/`Unpin`, `Box`, `Rc`/`Weak`, `Cell`, `RefCell`, and `Cow` are manifest-backed. `Arc` and thread-safety relationships continue in Phase 6; async pinning and projection are revisited in Phase 7.
