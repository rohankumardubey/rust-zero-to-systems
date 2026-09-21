# Ownership and memory baseline

Primary baselines: Rust Book ownership chapters, Rust Reference destruction/behavior chapters, std smart-pointer APIs, and the Rustonomicon for unsafe-adjacent models.

Audit: stack/heap; ownership/moves; `Copy`/`Clone`; shared/mutable borrowing; reborrowing; partial moves; drop scopes; dangling prevention; slices/NLL/RAII/`Drop`; `Box`/`Rc`/`Arc`/`Weak`; `Cell`/`RefCell`/`UnsafeCell`; `Cow`; `Pin`/`Unpin`; lifetimes, elision, bounds, `'static`, subtyping, variance, HRTBs.

Status: Phase 3 is in progress. Stack/heap intuition, moves, `Copy`/`Clone`, shared and mutable borrows, reborrowing, partial moves, drop scopes, RAII/`Drop`, NLL, basic lifetime relationships, `Box`, `Rc`/`Weak`, `Cell`, `RefCell`, and `Cow` are manifest-backed. `Pin`/`Unpin`, advanced lifetime relationships, and thread-safe smart pointers remain pending; advanced links continue in Phases 6–8.
