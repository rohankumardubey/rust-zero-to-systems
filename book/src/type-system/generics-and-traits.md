# Generics and traits

Generics let one implementation work with several concrete types while retaining static type
checking. A trait names the behavior an implementation promises. Trait bounds state which behavior a
generic function uses, and associated types model one type chosen by each implementation.

Generic items are compiled with concrete types at use sites. This is commonly called
monomorphization; it is useful to understand, but source code alone does not establish a runtime
performance claim. Measure a real workload before comparing implementations.

```rust
{{#include ../../../03-types-generics-traits/examples/001_generic_functions_and_types.rs}}
```

```rust
{{#include ../../../03-types-generics-traits/examples/002_trait_definitions_and_defaults.rs}}
```

```rust
{{#include ../../../03-types-generics-traits/examples/003_trait_bounds_and_where.rs}}
```

An associated type is appropriate when an implementation determines one natural item type. The
standard library's `Iterator::Item` is a familiar example. A trait with generic parameters instead
permits multiple implementations for different parameter combinations.

```rust
{{#include ../../../03-types-generics-traits/examples/004_associated_types.rs}}
```

Continue with [dispatch and trait-based API inputs](bounds-dispatch-and-conversions.md). Advanced
Phase 4 topics will extend this foundation with coherence, blanket implementations, GATs, marker
traits, and unsized types.
