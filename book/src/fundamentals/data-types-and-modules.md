# Data types and modules

Structs collect related fields into a named type. Enums represent one of a finite set of variants, which is why they pair naturally with `match`. An `impl` block attaches associated functions (such as constructors) and methods whose first parameter is `self`, `&self`, or `&mut self`.

Modules establish namespaces and privacy boundaries. Items are private by default; `pub` exposes only the intentional part of an API. Documentation comments (`///`) describe public items for rustdoc. Packages can contain one or more crates; this repository’s lesson package exports a library and several example binaries.

```rust
{{#include ../../../01-core-rust/examples/009_structs_enums_methods.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/010_modules_visibility_paths.rs}}
```

Next: [attributes, configuration, and callbacks](attributes-and-callbacks.md).

