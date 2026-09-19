# Rust Zero to Systems — persistent project specification

## Mission

Build an executable Rust encyclopedia, textbook, interview reference, and systems-engineering laboratory. The path begins with stable Rust fundamentals and progresses through ownership, types, the standard library, concurrency, async, unsafe Rust, networking, storage, databases, distributed systems, ecosystem technologies, interviews, and nine substantial projects.

The repository is educational, source-backed, runnable, and auditable. It is not a random snippet collection and must not imply that an incomplete inventory is comprehensive.

## Baseline and compatibility

- Default channel: stable Rust.
- Tested baseline at bootstrap: `rustc 1.98.1`, `cargo 1.98.1`, `rustup 1.29.1`.
- Edition: Rust 2024; workspace resolver: 3.
- Bootstrap/ecosystem review date: 2026-09-19.
- Stable, nightly-only, experimental, deprecated, ecosystem, and historical material must be labeled distinctly.
- Nightly examples live outside the stable workspace or behind explicit tooling and never break stable CI.
- Audit language coverage against the Rust Reference, The Rust Programming Language, Edition Guide, Cargo Book, rustc documentation, Rustonomicon, standard-library docs, Async Rust resources, and official documentation for demonstrated libraries. Write original explanations and link to primary sources.

## Architecture

Subject areas are top-level ordered directories, not a single beginner/intermediate/advanced hierarchy:

```text
01-core-rust/                  11-networking/                  21-webassembly/
02-ownership-borrowing/        12-unsafe-rust/                 22-ffi/
03-types-generics-traits/      13-macros/                      23-embedded/
04-patterns-control-flow/      14-testing/                     24-data-engineering/
05-collections-iterators/      15-performance/                 25-rust-internals/
06-error-handling/             16-files-serialization/         26-design-patterns/
07-lifetimes-smart-pointers/   17-cli/                         27-algorithms-data-structures/
08-standard-library/           18-web-backend/                 28-distributed-systems/
09-concurrency/                19-databases/                   29-frameworks/
10-async-rust/                 20-observability/               30-interview-prep/
mental-models/ projects/ tools/ book/ coverage/ docs/ .github/
```

Directories are added when their phase starts; empty placeholder forests are avoided. Small lesson packages join the root workspace. Projects or framework demonstrations may be excluded or nested independently when dependency, target, toolchain, or service constraints make a single workspace counterproductive.

## Lesson conventions

Whenever reasonable, one Rust concept equals one focused source file, using ordered descriptive names such as `003_mutability.rs`. A lesson should be runnable with `cargo run -p <package> --example <name>` (or the command in its header). Progressive files are encouraged for a concept with meaningful stages.

Every meaningful educational example begins with concise inner documentation covering:

- title, level, and category;
- what and why;
- when to use and when to avoid;
- prerequisites and related concepts;
- exact run command;
- for major topics: mechanism, common mistakes, performance, and next topics.

Unsafe lessons additionally state why unsafe is necessary, all caller/author invariants, possible undefined behavior, and the safe boundary. Systems prototypes prominently say **EDUCATIONAL IMPLEMENTATION — NOT PRODUCTION READY**, then list assumptions, limitations, and production requirements.

Book pages explain and cross-link concepts, then include real sources with mdBook include directives instead of copying snippets. Use Mermaid or maintainable text diagrams for ownership, borrowing, `Rc`, `Arc`, mutexes, future polling, executors, thread pools, TCP/async servers, WAL/LSM, Raft, the distributed KV store, and the query engine.

## Coverage contract

`coverage/manifest.toml` is the machine-readable source of truth. Each inventoried concept has:

- a stable unique dotted `id` and category;
- level (`beginner`, `intermediate`, or `advanced`);
- lifecycle status (`planned`, `in_progress`, `complete`, or `blocked`);
- source and book documentation paths when implemented;
- prerequisites and related concept IDs;
- `tested` state and an optional stability classification.

The checker verifies schema values, unique IDs, repository-relative safe paths, declared files, complete-entry requirements, related/prerequisite references, and book navigation. Missing/incomplete inventory is reported honestly; completion percentages are computed from manifest entries, never hand-written. Coverage audit pages define the baseline/checklist for language, std, Cargo/tooling, ownership/memory, types, concurrency, async, unsafe, macros, networking, distributed systems, and the curated ecosystem. Phase 19 reconciles every baseline against the manifest.

## Required scope

### Language and memory

Cover lexical structure, comments/docs, identifiers/keywords/literals, bindings, mutability, constants/statics, shadowing, scalar/compound types, tuples/arrays/slices/strings, references, functions, statements/expressions/blocks/operators, all stable control flow and patterns, structs/enums/unions, methods/associated functions, crates/packages/modules/visibility/paths, attributes/cfg, conversions, aliases, never type, and function pointers.

Treat ownership as a deep sequence: stack/heap, moves, `Copy`/`Clone`, shared and mutable borrowing, reborrowing, partial moves, drop scopes, dangling prevention, slices, NLL, RAII/`Drop`, `Box`, `Rc`, `Arc`, `Weak`, `Cell`, `RefCell`, `UnsafeCell` concepts, `Cow`, `Pin`/`Unpin`, and smart-pointer design. Lifetimes cover motivation and relationships—not syntax alone—including elision, multiple/struct lifetimes, bounds, `'static`, NLL, subtyping, variance, HRTBs, and advanced relationships.

### Type system, collections, and errors

Cover generics; trait definitions/impls/bounds/`where`; default methods; associated types/constants; supertraits; blanket impls; coherence/orphan rules; static/dynamic dispatch; trait objects/dyn compatibility; auto/marker traits; `Send`, `Sync`, `Sized`, `?Sized`; closure traits; conversions and borrowing traits; const generics; GATs; `impl Trait`; DSTs, coercions, fat pointers/vtables, monomorphization, `PhantomData`, variance, and typestate.

Cover `Vec`, `VecDeque`, linked-list tradeoffs, maps/sets, heaps, iterator ownership modes, laziness, core adaptors, collection, and custom iterators. Cover `Option`, `Result`, panic/unwrap/expect, `?`, custom/library/application errors, source chains/conversion, `thiserror`, and `anyhow` with tradeoffs.

### Standard library and tooling

Audit concept/API families across collections, fs/io/net, sync/thread, process, path/time/env, mem/ptr/slice/str/ffi, future/task/pin/panic, formatting/comparison/operators. Cover Cargo manifests, features, profiles, workspaces, build scripts, publishing concepts, rustfmt, Clippy, rustdoc, rustup, cross compilation, and edition migration. Avoid a file per trivial method.

### Flagship concurrency and async

Concurrency covers spawn/join/move/scoped threads, park/unpark, TLS; `Send`/`Sync`/`Arc`; mutexes, rwlocks, condvars, barriers, `Once`/`OnceLock`/`LazyLock`; std and Crossbeam channels, bounded/rendezvous behavior and backpressure; atomics, CAS loops, all orderings, happens-before and fences; contention, deadlock, starvation, false sharing/cache lines, lock-/wait-free concepts, ABA, reclamation, and work stealing; plus maintained Rayon, Crossbeam, and parking_lot where valuable.

Async covers syntax plus `Future`, `Poll`, `Context`, `Waker`/`RawWaker` concepts, `Pin`, executors/reactors/schedulers, cooperative scheduling, spawning/join/select, cancellation safety, time, async channel families, semaphores/notify, streams/sinks, backpressure, shutdown, blocking and `spawn_blocking`, `Send` and non-`Send` futures, and task `'static` requirements. Tokio receives extensive treatment; build a tiny educational executor; compare threads and async.

### Unsafe, macros, testing, performance, internals

Unsafe covers blocks/functions/traits, raw pointers/arithmetic, aliasing/alignment/provenance, layout/padding/reprs, `MaybeUninit`, `ManuallyDrop`, `NonNull`, `UnsafeCell`, unions, FFI, soundness/UB, safe abstractions, and Miri. Macros cover declarative fragments/repetition/recursion/hygiene and a procedural macro workspace using token streams, `syn`, and `quote` for derive, attribute, and function-like macros.

Testing covers unit/integration/doc tests, fixtures/helpers, property testing/proptest, fuzzing/cargo-fuzz, trybuild, concurrency/loom concepts, mocking, and benchmarks. Broken lessons use `compile_fail` or isolated fixtures. Performance covers profiles/LTO/PGO, allocation/clones/layout/cache locality/zero-copy/SIMD, Criterion, profiling/flamegraphs, throughput and latency distributions, and contention; make no speed claim without measurement.

Internals trace source → tokens → AST → HIR → type/trait checking → MIR/borrow checking → LLVM IR → machine code, plus macro expansion, monomorphization, vtables/fat pointers, enum/niche/ZST layout, ABI, linking, and incremental compilation. Mark nightly inspection commands.

### Networking, persistence, and ecosystem

Networking covers sockets, TCP/UDP, framing/protocol design, HTTP clients/servers, pools, timeouts/retries/backoff, TLS/DNS concepts, WebSocket, RPC/gRPC, heartbeats, serialization, and graceful shutdown using maintained Tokio/Hyper/Reqwest/Tonic/Prost where appropriate.

Databases cover SQLite/PostgreSQL and MySQL concepts; SQLx, Diesel, and SeaORM where appropriate; pools, transactions, prepared statements, and migrations. Storage internals cover WAL, pages/indexing/B-trees, LSM trees, MemTables/SSTables, checksums, compaction, recovery, and caching without outsourcing educational core engines.

Web/backend comparisons implement equivalent health/user endpoints in maintained Axum, Actix Web, and Rocket if appropriate, including JSON, middleware/state/errors, tracing, DB access, and shutdown. Curate frontend/full-stack (Leptos/Yew/Dioxus), desktop (Tauri), and game (Bevy) material by maintenance status. WebAssembly covers targets, wasm-bindgen/wasm-pack, JS/browser boundaries, WASI/server-side concepts, and component-model concepts. Embedded covers `no_std`, embedded-hal, peripherals, interrupts/MMIO concepts, and Embassy with hardware labels. FFI spans C, C++, Python, and Node via appropriate maintained tools.

Data engineering includes Arrow, Parquet, DataFusion, Polars, Kafka clients, object storage, and streaming concepts, with CSV→transform→aggregate→Parquet and Kafka→transform→sink pipelines. Observability covers `log`, tracing/spans/events, levels, metrics/Prometheus, OpenTelemetry, request IDs, and distributed tracing. `ECOSYSTEM.md` and `docs/ecosystem-status.md` curate relevant libraries by purpose, official source, example, alternatives, and review date; they never attempt all of crates.io.

### Algorithms, mental models, comparisons, interviews

Implement educational stacks, queues/deques, linked lists, heaps, BSTs, tries, graphs, union-find, and LRU; binary search/sorting, BFS/DFS/topological sort, Dijkstra/Bellman-Ford/A*, sliding-window/two-pointer/prefix-sum, recursion/backtracking, DP, greedy, and heap algorithms with time/space complexity.

Create 5–10 minute mental-model pages for ownership, the borrow checker, lifetimes, Box/Rc/Arc, Send/Sync, threads vs async, future polling, pinning, memory ordering, trait objects/vtables, dispatch, and zero-cost abstractions. Add tradeoff guides for String/&str, Copy/Clone, smart pointers, interior mutability, synchronization, memory/channels, thread/task/async, Iterator/Stream, Option/Result, panic/Result, dispatch, REST/gRPC, TCP/UDP, database libraries, web frameworks, and Arc<Mutex>/channels—without arbitrary winners.

Interview preparation has linked beginner/intermediate/advanced Rust, concurrency, async, unsafe, systems, networking, database, distributed-system, and coding-exercise sections. Every question links to executable lessons or docs.

### Distributed systems

Teach logical/Lamport/vector clocks; replication and primary/replica; quorum and consistency models; CAP; consensus, leader election and Raft concepts; membership/heartbeats/failure detection; retries/idempotency; partitioning/sharding/consistent hashing; WAL/replication logs/snapshots; backpressure/load balancing/circuit breakers; distributed locks, gossip, and discovery. Use small crates where concepts exceed a focused example.

### Nine progressive projects

Each project has a manifest, README, source, and tests:

1. CLI search tool: Clap, filesystem, regex, iterators, errors, tests, optional parallelism.
2. Multithreaded HTTP server: `TcpListener`, parsing, pool, routing, shutdown.
3. Tokio chat server: connections, tasks/channels/broadcast, cancellation, backpressure, shutdown.
4. Mini Redis: GET/SET/DEL/EXISTS/EXPIRE/TTL, concurrent memory, optional persistence.
5. Mini LSM database: own WAL, MemTable, SSTable, indexes, checksums, compaction, recovery.
6. Mini message broker: topics/partitions, producers/consumers, append log, offsets, persistence.
7. Deterministically tested Raft simulator: roles, terms, RequestVote, AppendEntries, elections, heartbeats, replication, commit index.
8. Distributed KV: storage, networking, partitioning, documented consistent-hashing alternative, replication, health, retries, failure simulation.
9. Mini distributed query engine: query/SQL parser → logical plan → physical plan → partitioned execution → aggregation; begin with CSV/Parquet and SELECT/WHERE/GROUP BY/COUNT/SUM, then distributed execution.

## mdBook and navigation

The book provides hierarchy, search, Rust highlighting, theme switching, previous/next links, cross-links, source includes, diagrams, learning/project roadmaps, and source links. The root README is the dashboard: audience, prerequisites, quick start, map, progression, concept/framework/project indexes, book/testing commands, and manifest-derived coverage status. A user looking for `Arc` should immediately find concurrency and smart-pointer routes.

## Quality and dependency policy

Before adding a dependency, ask whether std suffices, whether the crate is maintained, whether it is educationally/currently relevant, and whether a smaller choice exists. Commit a lockfile for reproducible repository checks. Normal quality gate:

```bash
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p coverage-check -- coverage/manifest.toml
mdbook build book
```

CI runs all gates and builds the book; GitHub Pages publishes only after successful validation. External services, targets, tools, and hardware must be documented and kept out of the default stable gate where necessary.

## Phase plan and completion

The canonical status is `docs/PHASES.md`: Phase 0 audit/architecture/instructions; 1 workspace/tooling/book/CI/checker; 2 fundamentals; 3 ownership/lifetimes/smart pointers; 4 type system/collections/iterators; 5 errors/std/testing; 6 concurrency; 7 async/Tokio; 8 unsafe/macros/internals; 9 networking/observability; 10 databases/storage; 11 web/frameworks/WASM/FFI/embedded; 12 data/performance; 13 algorithms/interviews; 14 distributed concepts; 15 projects 1–3; 16 projects 4–6; 17 projects 7–9; 18 book expansion/cross-linking; 19 complete coverage audit; 20 final quality pass.

A phase is complete only when its promised code/docs/tests/coverage are present and applicable checks pass. Use `NOT STARTED`, `IN PROGRESS`, `COMPLETE`, or `BLOCKED`. Each run updates coverage and status, validates, stops, and reports phase/checks/failures/next phase briefly.

