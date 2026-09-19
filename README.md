# Rust Zero to Systems

An executable Rust learning and reference system: language fundamentals, ownership, type systems, concurrency, async, unsafe Rust, networking, storage, databases, distributed systems, ecosystem labs, interview preparation, and progressive projects.

The repository is being built phase by phase. Coverage numbers describe only concepts currently inventoried in `coverage/manifest.toml`; they are not a claim that Rust or its ecosystem is already fully covered.

## Who this is for

- newcomers who want runnable, focused lessons;
- Rust developers revising a specific concept;
- systems engineers studying memory, concurrency, storage, and distributed design;
- interview candidates who want answers linked to executable evidence;
- maintainers looking for small, testable reference implementations.

## Prerequisites and quick start

Install stable Rust with `rustup`. The bootstrap baseline is Rust 1.98.1 and edition 2024.

```bash
cargo run -p core-rust-lessons --example 001_hello_world
cargo test --workspace
cargo run -p coverage-check -- coverage/manifest.toml
```

Install mdBook separately, then browse the knowledge base:

```bash
cargo install mdbook
mdbook serve book --open
```

## Find a concept

| If you need… | Start here |
|---|---|
| syntax, values, or control flow | `01-core-rust/`, then `04-patterns-control-flow/` |
| ownership or borrowing | `02-ownership-borrowing/` and `mental-models/ownership.md` |
| traits or generics | `03-types-generics-traits/` |
| `Box`, `Rc`, `Arc`, or lifetimes | `07-lifetimes-smart-pointers/`; `Arc` continues in `09-concurrency/` |
| mutexes, channels, or atomics | `09-concurrency/` |
| futures or Tokio | `10-async-rust/` |
| TCP, HTTP, or RPC | `11-networking/` |
| unsafe invariants | `12-unsafe-rust/` |
| storage and databases | `19-databases/` and Projects 4–5 |
| replication or Raft | `28-distributed-systems/` and Projects 7–8 |
| interview review | `30-interview-prep/` |

Directories appear when their phase contains real lessons. See [ROADMAP.md](ROADMAP.md) for the learning route and [docs/PHASES.md](docs/PHASES.md) for delivery status.

## Repository map

- `01-` through `30-`: subject-focused executable lessons and labs.
- `book/`: mdBook knowledge base; canonical code is included from lesson sources.
- `coverage/`: machine manifest plus authoritative-baseline audit pages.
- `mental-models/`: eventual 5–10 minute revision guides.
- `projects/`: nine progressively harder systems projects.
- `tools/coverage-check/`: validates coverage paths, references, state, and book navigation.
- `docs/`: specification, decisions, phase tracking, and ecosystem review.

## Learning and systems progression

```text
Cargo → syntax → ownership → data modeling/errors → traits/iterators → lifetimes
      → concurrency → async/Tokio → networking → unsafe/internals
      → performance → storage/databases → distributed systems → projects
```

Suggested project milestones are listed in [ROADMAP.md](ROADMAP.md). The final project ladder runs from a CLI search tool through a distributed query engine.

## Quality gate

```bash
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p coverage-check -- coverage/manifest.toml
mdbook build book
```

CI runs the same stable-Rust checks. GitHub Pages publishes the validated book.

## Coverage status

Run the checker for the current derived count:

```bash
cargo run -p coverage-check -- coverage/manifest.toml
```

Audit categories are listed under `coverage/`. Stable-language, standard-library, Cargo/tooling, systems-topic, and curated-ecosystem coverage remain separate measures. No percentage is hardcoded here.

## Framework and project indexes

The curated framework catalog starts in [ECOSYSTEM.md](ECOSYSTEM.md), with dated review records in [docs/ecosystem-status.md](docs/ecosystem-status.md). Project implementation begins in Phase 15; the contracts for all nine projects live in [docs/PROJECT_SPEC.md](docs/PROJECT_SPEC.md).

## Project state

Phases 0–1 are complete. Phase 2 fundamentals are in progress; begin with the expanded [Fundamentals](book/src/fundamentals/index.md) path. Contributions should follow [CONTRIBUTING.md](CONTRIBUTING.md) and the persistent conventions in [AGENTS.md](AGENTS.md).
