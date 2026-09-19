# Repository operating instructions

This repository is built incrementally. On every implementation run:

1. Read `AGENTS.md`, `docs/PROJECT_SPEC.md`, `docs/DECISIONS.md`, and `docs/PHASES.md`.
2. Inspect the current tree and `git status`; preserve unrelated work and established conventions.
3. Work only on the next incomplete phase (or the phase explicitly requested). Do not redo completed work.
4. Prefer one focused, independently runnable Rust example per concept. Keep intentionally failing code in `compile_fail` docs or isolated test fixtures.
5. Start educational examples with the metadata/documentation template defined in the project specification.
6. Use stable Rust by default. Isolate and label nightly, platform, hardware, and external-service examples.
7. Update `coverage/manifest.toml`, relevant coverage audit pages, book navigation, and phase status with each lesson.
8. Do not mark a concept or phase complete unless its implementation, documentation, and applicable tests exist.
9. Run the checks applicable to touched code. The full quality gate is `cargo fmt --check`, `cargo check --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo run -p coverage-check -- coverage/manifest.toml`, and `mdbook build book`.
10. Record durable architectural changes in `docs/DECISIONS.md`; keep the final report concise: phase, validation, failures, next phase.

Never claim comprehensive coverage unless the manifest and the Phase 19 audit support it. Ecosystem coverage is curated and date-sensitive.

