# Learning and delivery roadmap

## Learning path

```text
Environment / Cargo
        ↓
Syntax and core values                 exercise: focused examples
        ↓
Ownership and borrowing               exercise: owned text transformer
        ↓
Structs, enums, patterns, errors       exercise: domain model + parser
        ↓
Generics, traits, collections, iterators
        ↓                               exercise: generic data pipeline
Lifetimes and smart pointers
        ↓
Concurrency                            exercise: bounded worker pool
        ↓
Async Rust and Tokio                   exercise: cancellable service
        ↓
Networking and observability           Project 2 → Project 3
        ↓
Unsafe Rust and compiler internals      exercise: safe wrapper with invariants
        ↓
Performance                            exercise: benchmark before optimizing
        ↓
Storage and databases                  Project 4 → Project 5
        ↓
Distributed systems                    Project 6 → Project 7 → Project 8
        ↓
Distributed query processing           Project 9
```

## Systems project ladder

1. CLI search tool
2. Multithreaded HTTP server
3. Async chat server
4. Mini Redis
5. Mini LSM database
6. Mini message broker
7. Raft simulator
8. Distributed KV store
9. Mini distributed query engine

Every systems project is an **EDUCATIONAL IMPLEMENTATION — NOT PRODUCTION READY** and must describe its assumptions, omissions, and the additional work production software requires.

## Delivery roadmap

Delivery follows the 21 phases in [docs/PHASES.md](docs/PHASES.md). A phase moves to complete only when code, book pages, manifest entries, tests, and applicable validation agree. Work does not skip ahead at the cost of the quality gate.

