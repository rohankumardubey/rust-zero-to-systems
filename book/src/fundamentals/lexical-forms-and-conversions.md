# Lexical forms and conversions

Comments explain intent to people, while documentation comments (`///` and `//!`) become rustdoc. Rust keywords cannot normally be identifiers, but `r#` raw identifiers make interoperating with protocol fields or older APIs possible without giving up the correct external name.

Primitive casts using `as` are explicit, but may be lossy. Prefer `From` for well-defined infallible conversions and `TryFrom` when input must be range-checked.

```rust
{{#include ../../../01-core-rust/examples/012_comments_raw_identifiers.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/013_conversions_and_casts.rs}}
```

