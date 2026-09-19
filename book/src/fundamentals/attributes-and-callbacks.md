# Attributes, configuration, and callbacks

Attributes annotate items for tools or the compiler. `#[must_use]` asks callers not to silently discard a meaningful return value; `#[cfg(test)]` compiles test-only code when the test harness is built. `cfg!` evaluates a compile-time configuration predicate to a boolean, which is useful when one source file has harmless platform-specific messaging. More substantial platform implementations should use `#[cfg(...)]` to compile only the appropriate items.

A type alias gives a domain-relevant name to an existing type without creating a new distinct type. A function pointer such as `fn(i32, i32) -> i32` can call a plain function but cannot capture surrounding variables; later closure and trait lessons cover callbacks that do.

```rust
{{#include ../../../01-core-rust/examples/011_attributes_cfg_aliases_function_pointers.rs}}
```

Remaining Phase 2 work includes lexical rules, keywords, additional conversion and pattern topics, unit/tuple structs, unions, package/crate organization in depth, and the never type.

