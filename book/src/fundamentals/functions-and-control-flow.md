# Functions and control flow

Functions declare parameter and return types. Rust is expression-oriented: a block can evaluate to its final expression, so a function can return a value without `return`. A semicolon changes an expression into a statement, which is a common early source of mismatched-type errors.

Use `match` when the handled forms should be exhaustive. `if let` and `while let` focus on a single pattern, while `let else` performs an early diverging exit when a pattern does not match. `for` is usually the clearest choice for a finite iterator; `while` and `loop` remain useful for condition-driven and explicitly repeated work.

```rust
{{#include ../../../01-core-rust/examples/007_functions_statements_expressions.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/008_control_flow_patterns.rs}}
```

Next: [data types and modules](data-types-and-modules.md).

