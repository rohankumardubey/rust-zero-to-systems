# Bindings and scalar values

Rust local bindings are immutable unless declared with `mut`. That small default makes state changes opt-in and visible during review. Shadowing is different: a second `let` creates a new binding, possibly with a different type, while the old binding stops being accessible by that name.

Constants are named compile-time values and must have a type. An immutable `static` is one value at a stable address for the program. Mutable globals are deliberately absent from this lesson because they introduce safety and synchronization concerns that belong in later material.

Scalar values include signed and unsigned integers, floating-point values, booleans, and Unicode scalar values (`char`). Literal spelling can make a representation choice clear, such as `0xff_u16`.

```rust
{{#include ../../../01-core-rust/examples/002_variables_mutability.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/003_constants_statics_shadowing.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/004_scalar_types_literals_operators.rs}}
```

Next: [compound values and text](compound-values-and-text.md).

