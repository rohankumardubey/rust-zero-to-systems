# Packages and crates

Cargo reads a package manifest, `Cargo.toml`, to build one or more crates. This lesson package contains a library crate (`src/lib.rs`) and example binary crates (`examples/*.rs`). The binary uses `use core_rust_lessons::greeting;` to import one public library item into scope, while the library’s full module path remains the canonical API.

This distinction matters later in a workspace: a workspace coordinates several packages, but it is not itself a package unless it also declares one.

```rust
{{#include ../../../01-core-rust/examples/018_packages_crates_and_use.rs}}
```

