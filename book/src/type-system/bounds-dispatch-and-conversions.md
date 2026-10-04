# Trait bounds, dispatch, and conversion APIs

Trait bounds describe the capabilities generic code requires. Writing complex requirements in a
`where` clause can make a signature easier to scan. Keep the bounds minimal so callers retain as
many valid types as possible.

A generic function normally uses static dispatch: the concrete implementation is selected during
compilation. A `dyn Trait` trait object uses dynamic dispatch and supports heterogeneous values
behind one interface. Trait objects have runtime representation and object-safety constraints;
they are useful when runtime heterogeneity matters, not as a universal replacement for generics.

```rust
{{#include ../../../03-types-generics-traits/examples/003_trait_bounds_and_where.rs}}
```

```rust
{{#include ../../../03-types-generics-traits/examples/005_static_and_dynamic_dispatch.rs}}
```

`Into<T>` is useful for accepting values that can become an owned `T`; `AsRef<T>` is useful for a
borrowed view. Related traits such as `From`, `TryFrom`, `Borrow`, `Deref`, and `AsMut` have distinct
contracts and will receive focused coverage in later Phase 4 lessons.

```rust
{{#include ../../../03-types-generics-traits/examples/006_conversion_and_borrowing_traits.rs}}
```
