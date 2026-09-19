# Architectural decisions

## ADR-001: Subject-first numbered layout

**Status:** Accepted — 2026-09-19

Use numbered subject directories, reserving difficulty labels for lesson metadata. This makes lookup stable (for example, `Arc` belongs in concurrency/smart pointers) while retaining a guided order.

## ADR-002: Rust 2024 workspace with resolver 3

**Status:** Accepted — 2026-09-19

Use stable Rust, edition 2024, and Cargo resolver 3. The bootstrap machine and official release channel both report Rust 1.98.1. `rust-toolchain.toml` follows `stable` rather than pinning an aging patch release; the tested version is recorded in docs and CI output.

## ADR-003: Selective workspace membership

**Status:** Accepted — 2026-09-19

Keep ordinary lesson crates and repository tools in the root workspace. Allow independent/nested projects for incompatible toolchains, targets, large framework dependency graphs, or external services. Document each exclusion when introduced.

## ADR-004: Manifest-driven, honest coverage

**Status:** Accepted — 2026-09-19

The TOML manifest contains inventoried concepts, not aspirational percentages. Audit checklists may be broader while inventory is being built. Only `complete` records count as covered, and the checker computes totals.

## ADR-005: Source inclusion in mdBook

**Status:** Accepted — 2026-09-19

Book pages include canonical `.rs` files with mdBook directives. Explanatory prose may show tiny independent fragments, but executable examples are not manually duplicated.

## ADR-006: Coverage checker is a workspace tool

**Status:** Accepted — 2026-09-19

Implement validation in Rust using `serde` and `toml`. This exercises the repository's own toolchain, keeps CI portable, and permits schema growth with unit tests.

## ADR-007: Do not commit empty subject directories

**Status:** Accepted — 2026-09-19

The architecture is specified now, while directories appear with real content during their phase. This avoids misleading scaffolds and TODO-only lessons.

