# Lifetime relationships

Lifetimes describe how long references remain valid relative to one another. They are not timers and do not extend a value’s lifetime; annotations express a relationship the compiler can verify. The most common case is a function returning one of its input borrows, which needs one named relationship between all relevant references.

Structs that store references carry the same relationship in their type. Elision rules infer common function and method cases, while `'static` is appropriate only for data valid for the full program, such as a string literal.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/001_lifetime_annotations.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/002_struct_lifetimes.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/003_elision_and_static.rs}}
```

Later Phase 3 material will cover multiple lifetime relationships, bounds, subtyping, variance, and higher-ranked trait bounds.

