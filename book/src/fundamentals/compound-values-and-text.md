# Compound values and text

Tuples have a fixed number of elements that may have different types. Arrays have one element type and a length encoded in their type. A slice, written `&[T]`, is a borrowed view of contiguous elements; the caller keeps ownership of the underlying array or collection.

Text has the same distinction. `str` is an unsized UTF-8 string slice type, so Rust normally uses it through `&str`. `String` owns a growable UTF-8 buffer. Accepting `&str` avoids an allocation for read-only input; returning `String` transfers owned output to the caller. Neither supports safe integer indexing by character position because UTF-8 uses variable-width encoding.

```rust
{{#include ../../../01-core-rust/examples/005_tuples_arrays_slices.rs}}
```

```rust
{{#include ../../../01-core-rust/examples/006_str_and_string.rs}}
```

Next: [functions and control flow](functions-and-control-flow.md).

