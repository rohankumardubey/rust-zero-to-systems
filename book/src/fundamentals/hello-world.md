# Hello, world!

Rust binaries begin at `main`. `println!` is a macro (the `!` distinguishes macro invocation from a function call) that writes formatted text to standard output. This first lesson also calls a tiny library function, proving examples and library tests share the workspace correctly.

Run it from the repository root:

```bash
cargo run -p core-rust-lessons --example 001_hello_world
```

The source below is included directly from the runnable file:

```rust
{{#include ../../../01-core-rust/examples/001_hello_world.rs}}
```

Next: functions and expressions will be implemented in Phase 2.

