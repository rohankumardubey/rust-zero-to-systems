# Additional data forms and divergence

Tuple structs are convenient newtypes: `Milliseconds(u64)` remains distinct from a bare `u64` in the type system. Unit structs, such as a format marker, are zero-sized values that communicate a type-level choice without allocating storage.

The never type, `!`, describes computations that cannot produce a normal result. A branch that panics or otherwise diverges can type-check where another branch produces a value. That does not make panics routine control flow: expected failures should normally be modeled with `Result`.

```rust
{{#include ../../../01-core-rust/examples/014_unit_and_tuple_structs.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/015_never_type_and_diverging_control_flow.rs}}
```

Union field access is intentionally deferred to the unsafe section, where the representation invariant and undefined-behavior boundary can be explained together.

