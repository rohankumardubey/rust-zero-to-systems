# Curated Rust ecosystem

This is a navigation catalog, not an endorsement ranking and not an attempt to index crates.io. A crate receives a detailed entry only when its lesson phase reviews maintenance, documentation, fit, and alternatives. Review evidence belongs in [docs/ecosystem-status.md](docs/ecosystem-status.md).

| Category | Candidates for phase review | Planned subject area |
|---|---|---|
| async/runtime | Tokio, async-std/smol as comparison points | `10-async-rust/` |
| concurrency | Crossbeam, Rayon, parking_lot, Loom | `09-concurrency/`, `14-testing/` |
| HTTP/web/RPC | Hyper, Reqwest, Axum, Actix Web, Rocket, Tonic, Prost | `11-networking/`, `18-web-backend/`, `29-frameworks/` |
| serialization/parsing | Serde, serde_json, nom, pest | `16-files-serialization/` |
| CLI/config/errors | Clap, config/figment, thiserror, anyhow | `17-cli/`, `06-error-handling/` |
| tracing/metrics | tracing, metrics, Prometheus clients, OpenTelemetry | `20-observability/` |
| database/ORM | SQLx, Diesel, SeaORM, rusqlite | `19-databases/` |
| data/streaming | Arrow, Parquet, DataFusion, Polars, Kafka clients | `24-data-engineering/` |
| cloud/Kubernetes | official AWS SDK, kube | ecosystem labs |
| WebAssembly | wasm-bindgen, wasm-pack, WASI tooling | `21-webassembly/` |
| desktop/GUI/game | Tauri, egui/Iced, Bevy | `29-frameworks/` |
| embedded | embedded-hal, Embassy | `23-embedded/` |
| FFI | bindgen, cxx, PyO3/maturin, napi-rs | `22-ffi/` |
| regex/crypto/compression/time | regex, RustCrypto families, flate2/zstd, time/chrono | relevant subject labs |
| testing/fuzzing/benchmarking | proptest, cargo-fuzz, trybuild, Criterion | `14-testing/`, `15-performance/` |
| profiling | cargo-flamegraph and platform profilers | `15-performance/` |

Entries become authoritative only after their dated phase review.

