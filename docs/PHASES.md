# Phase tracker

Last updated: 2026-09-19

| Phase | Scope | Status | Evidence / exit criteria |
|---:|---|---|---|
| 0 | Audit, architecture, persistent instructions | COMPLETE | `AGENTS.md`, project specification, decisions, and tracker exist; empty-repository/toolchain audit recorded. |
| 1 | Workspace, tooling, mdBook, CI, coverage checker | COMPLETE | Workspace/tool configs, checker/tests, coverage baselines, book scaffold, README/roadmap, CI and Pages workflows; full local gate recorded below. |
| 2 | Core Rust fundamentals | COMPLETE | 18 focused examples cover the introductory stable-language sequence; manifest/book/tests are linked. Ownership/lifetimes and unsafe-only material move to their dedicated phases. |
| 3 | Ownership, borrowing, lifetimes, smart pointers | IN PROGRESS | Opening ownership and borrowing sequence is runnable, documented, and manifest-backed; lifetimes and smart pointers remain pending. |
| 4 | Generics, traits, type system, collections, iterators | NOT STARTED | Type-system and collection baselines reconciled. |
| 5 | Errors, standard library, testing | NOT STARTED | Error patterns, std families, and testing techniques covered. |
| 6 | Concurrency and memory model | NOT STARTED | Flagship concurrency sequence and maintained-library labs complete. |
| 7 | Async Rust and Tokio | NOT STARTED | Language/runtime internals, Tokio, and tiny executor complete. |
| 8 | Unsafe Rust, macros, compiler internals | NOT STARTED | Safety contracts, macro workspace, and internals path complete. |
| 9 | Networking and observability | NOT STARTED | Protocol/network and telemetry lessons complete. |
| 10 | Databases, storage, persistence | NOT STARTED | Client APIs and storage-engine foundations complete. |
| 11 | Web/frameworks, WASM, FFI, embedded | NOT STARTED | Curated ecosystem examples reviewed and isolated appropriately. |
| 12 | Data engineering and performance | NOT STARTED | Pipelines plus measurement-based performance material complete. |
| 13 | Algorithms and interview preparation | NOT STARTED | Implementations, complexity, and linked question bank complete. |
| 14 | Distributed-system concepts | NOT STARTED | Flagship distributed-systems concept sequence complete. |
| 15 | Projects 1–3 | NOT STARTED | CLI search, threaded HTTP, async chat meet project contract. |
| 16 | Projects 4–6 | NOT STARTED | Mini Redis, LSM, and broker meet project contract. |
| 17 | Projects 7–9 | NOT STARTED | Raft, distributed KV, and query engine meet project contract. |
| 18 | Documentation expansion and cross-linking | NOT STARTED | Book, diagrams, mental models, comparisons, indexes integrated. |
| 19 | Complete coverage audit | NOT STARTED | All defined authoritative baselines reconciled with manifest. |
| 20 | Final quality pass | NOT STARTED | Full clean gate, link/navigation review, reproducibility review. |

## Validation log

- 2026-09-19 — Phase 0/1 bootstrap passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (3 tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (1/6 inventoried concepts complete), `mdbook build book`, and the bootstrap example run.
- 2026-09-19 — Phase 2 fundamentals tranche passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (3 tests), `cargo test -p core-rust-lessons --examples --locked` (10 lesson tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (16/16 inventoried concepts complete), and `mdbook build book`.
- 2026-09-19 — Phase 2 language-mechanics tranche passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (3 tests), `cargo test -p core-rust-lessons --examples --locked` (15 lesson tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (20/20 inventoried concepts complete), and `mdbook build book`.
- 2026-09-20 — Phase 2 completion tranche passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (3 tests), `cargo test -p core-rust-lessons --examples --locked` (18 examples; 17 lesson tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (23/23 inventoried concepts complete), and `mdbook build book`.
- 2026-09-20 — Phase 3 ownership/borrowing tranche passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (4 tests), `cargo test -p ownership-borrowing-lessons --examples --locked` (6 lesson tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (29/29 inventoried concepts complete), and `mdbook build book`.
- 2026-09-21 — Phase 3 lifetimes/smart-pointers tranche passed: `cargo fmt --check`, `cargo check --workspace --locked`, `cargo test --workspace --locked` (5 tests), `cargo test -p lifetimes-smart-pointers-lessons --examples --locked` (6 lesson tests), `cargo clippy --workspace --all-targets --locked -- -D warnings`, coverage validation (35/35 inventoried concepts complete), and `mdbook build book`.
